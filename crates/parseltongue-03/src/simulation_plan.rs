//! Simulation plan module for Tool 2
//!
//! Defines the structure for simulation plans that guide code change simulation

use parseltongue_01::types::{CoreError, CoreResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a complete simulation plan for a code change
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationPlan {
    /// Unique identifier for this simulation plan
    pub id: Uuid,

    /// Title of the simulation plan
    pub title: String,

    /// Description of what this plan simulates
    pub description: String,

    /// Steps in the simulation plan, ordered by execution
    pub steps: Vec<SimulationStep>,

    /// Dependencies between steps
    pub dependencies: HashMap<Uuid, Vec<Uuid>>,

    /// Metadata about the plan
    pub metadata: SimulationPlanMetadata,
}

/// Individual step in a simulation plan
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationStep {
    /// Unique identifier for this step
    pub id: Uuid,

    /// Phase identifier (A01, A02, B01, B02, C, D, etc.)
    pub phase: String,

    /// Title of this step
    pub title: String,

    /// Description of what this step does
    pub description: String,

    /// Type of step
    pub step_type: SimulationStepType,

    /// Expected inputs for this step
    pub inputs: Vec<String>,

    /// Expected outputs from this step
    pub outputs: Vec<String>,

    /// Validation criteria for this step
    pub validation_criteria: Vec<String>,

    /// Estimated execution time in seconds
    pub estimated_time_seconds: u64,

    /// Whether this step is critical (failure aborts simulation)
    pub is_critical: bool,
}

/// Types of simulation steps
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationStepType {
    /// Analysis step (A01, A02)
    Analysis,
    /// Impact assessment step (B01, B02)
    ImpactAssessment,
    /// Change application step (C)
    ChangeApplication,
    /// Validation step (D)
    Validation,
    /// Custom step type
    Custom(String),
}

/// Metadata for simulation plans
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationPlanMetadata {
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Estimated total execution time
    pub estimated_total_time_seconds: u64,

    /// Risk level of this simulation
    pub risk_level: RiskLevel,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Version of the simulation plan format
    pub version: String,
}

/// Risk levels for simulation plans
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl SimulationPlan {
    /// Create a new simulation plan
    pub fn new(title: String, description: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            title,
            description,
            steps: Vec::new(),
            dependencies: HashMap::new(),
            metadata: SimulationPlanMetadata {
                created_at: chrono::Utc::now(),
                estimated_total_time_seconds: 0,
                risk_level: RiskLevel::Medium,
                tags: Vec::new(),
                version: "1.0".to_string(),
            },
        }
    }

    /// Add a step to the simulation plan
    pub fn add_step(&mut self, step: SimulationStep) -> CoreResult<()> {
        // Validate the step
        if step.title.trim().is_empty() {
            return Err(CoreError::InvalidKey(
                "Step title cannot be empty".to_string(),
            ));
        }

        if step.estimated_time_seconds == 0 {
            return Err(CoreError::InvalidKey(
                "Step estimated time must be greater than 0".to_string(),
            ));
        }

        self.steps.push(step);
        self.update_estimated_total_time();
        Ok(())
    }

    /// Add a dependency between steps
    pub fn add_dependency(&mut self, step_id: Uuid, depends_on: Uuid) {
        self.dependencies
            .entry(step_id)
            .or_insert_with(Vec::new)
            .push(depends_on);
    }

    /// Get steps by phase
    pub fn steps_by_phase(&self, phase_prefix: &str) -> Vec<&SimulationStep> {
        self.steps
            .iter()
            .filter(|step| step.phase.starts_with(phase_prefix))
            .collect()
    }

    /// Get all phases in order
    pub fn phases(&self) -> Vec<String> {
        let mut phases: Vec<String> = self.steps.iter().map(|step| step.phase.clone()).collect();
        phases.sort();
        phases.dedup();
        phases
    }

    /// Validate the simulation plan
    pub fn validate(&self) -> CoreResult<Vec<String>> {
        let mut errors = Vec::new();

        // Check basic structure
        if self.title.trim().is_empty() {
            errors.push("Plan title cannot be empty".to_string());
        }

        if self.description.trim().is_empty() {
            errors.push("Plan description cannot be empty".to_string());
        }

        if self.steps.is_empty() {
            errors.push("Plan must have at least one step".to_string());
        }

        // Check for required phases
        let phases = self.phases();
        let has_phase_a = phases.iter().any(|p| p.starts_with('A'));
        let has_phase_b = phases.iter().any(|p| p.starts_with('B'));
        let has_phase_c = phases.iter().any(|p| p.starts_with('C'));
        let has_phase_d = phases.iter().any(|p| p.starts_with('D'));

        if !has_phase_a {
            errors.push("Plan must have phase A steps (analysis)".to_string());
        }
        if !has_phase_b {
            errors.push("Plan must have phase B steps (impact assessment)".to_string());
        }
        if !has_phase_c {
            errors.push("Plan must have phase C step (change application)".to_string());
        }
        if !has_phase_d {
            errors.push("Plan must have phase D step (validation)".to_string());
        }

        // Check dependencies
        for (step_id, dependencies) in &self.dependencies {
            // Check if step exists
            if !self.steps.iter().any(|step| step.id == *step_id) {
                errors.push(format!("Step {} not found but has dependencies", step_id));
            }

            // Check if dependencies exist
            for dep_id in dependencies {
                if !self.steps.iter().any(|step| step.id == *dep_id) {
                    errors.push(format!(
                        "Dependency step {} not found for step {}",
                        dep_id, step_id
                    ));
                }
            }
        }

        // Check for circular dependencies
        if self.has_circular_dependencies() {
            errors.push("Circular dependencies detected".to_string());
        }

        Ok(errors)
    }

    /// Check for circular dependencies
    fn has_circular_dependencies(&self) -> bool {
        // Simple depth-first search to detect cycles
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();

        for step in &self.steps {
            if !visited.contains(&step.id) {
                if self.has_cycle_dfs(step.id, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }
        false
    }

    /// Helper for cycle detection using DFS
    fn has_cycle_dfs(
        &self,
        step_id: Uuid,
        visited: &mut std::collections::HashSet<Uuid>,
        rec_stack: &mut std::collections::HashSet<Uuid>,
    ) -> bool {
        visited.insert(step_id);
        rec_stack.insert(step_id);

        if let Some(dependencies) = self.dependencies.get(&step_id) {
            for dep_id in dependencies {
                if !visited.contains(dep_id) {
                    if self.has_cycle_dfs(*dep_id, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(dep_id) {
                    return true;
                }
            }
        }

        rec_stack.remove(&step_id);
        false
    }

    /// Update estimated total time
    fn update_estimated_total_time(&mut self) {
        self.metadata.estimated_total_time_seconds =
            self.steps.iter().map(|s| s.estimated_time_seconds).sum();
    }

    /// Get all steps (immutable reference)
    pub fn steps(&self) -> &[SimulationStep] {
        &self.steps
    }

    /// Create a mock simulation plan for testing
    pub fn mock() -> Self {
        let mut plan = Self::new(
            "Test Simulation Plan".to_string(),
            "A mock plan for testing".to_string(),
        );

        // Add phase A steps
        let a01 = SimulationStep {
            id: Uuid::new_v4(),
            phase: "A01".to_string(),
            title: "Analyze Current Code".to_string(),
            description: "Analyze the current implementation".to_string(),
            step_type: SimulationStepType::Analysis,
            inputs: vec!["current_code".to_string()],
            outputs: vec!["analysis_result".to_string()],
            validation_criteria: vec!["analysis_complete".to_string()],
            estimated_time_seconds: 30,
            is_critical: true,
        };

        let a02 = SimulationStep {
            id: Uuid::new_v4(),
            phase: "A02".to_string(),
            title: "Analyze Target Changes".to_string(),
            description: "Analyze the proposed changes".to_string(),
            step_type: SimulationStepType::Analysis,
            inputs: vec!["proposed_code".to_string()],
            outputs: vec!["change_analysis".to_string()],
            validation_criteria: vec!["change_analysis_complete".to_string()],
            estimated_time_seconds: 45,
            is_critical: true,
        };

        // Add phase B steps
        let b01 = SimulationStep {
            id: Uuid::new_v4(),
            phase: "B01".to_string(),
            title: "Impact Analysis".to_string(),
            description: "Analyze impact of changes".to_string(),
            step_type: SimulationStepType::ImpactAssessment,
            inputs: vec!["analysis_result".to_string(), "change_analysis".to_string()],
            outputs: vec!["impact_report".to_string()],
            validation_criteria: vec!["impact_assessed".to_string()],
            estimated_time_seconds: 60,
            is_critical: true,
        };

        let b02 = SimulationStep {
            id: Uuid::new_v4(),
            phase: "B02".to_string(),
            title: "Dependency Analysis".to_string(),
            description: "Analyze dependencies affected".to_string(),
            step_type: SimulationStepType::ImpactAssessment,
            inputs: vec!["impact_report".to_string()],
            outputs: vec!["dependency_report".to_string()],
            validation_criteria: vec!["dependencies_analyzed".to_string()],
            estimated_time_seconds: 90,
            is_critical: true,
        };

        // Add phase C step
        let c = SimulationStep {
            id: Uuid::new_v4(),
            phase: "C".to_string(),
            title: "Apply Changes".to_string(),
            description: "Apply the code changes".to_string(),
            step_type: SimulationStepType::ChangeApplication,
            inputs: vec!["dependency_report".to_string()],
            outputs: vec!["applied_changes".to_string()],
            validation_criteria: vec!["changes_applied".to_string()],
            estimated_time_seconds: 120,
            is_critical: true,
        };

        // Add phase D step
        let d = SimulationStep {
            id: Uuid::new_v4(),
            phase: "D".to_string(),
            title: "Validate Changes".to_string(),
            description: "Validate the applied changes".to_string(),
            step_type: SimulationStepType::Validation,
            inputs: vec!["applied_changes".to_string()],
            outputs: vec!["validation_result".to_string()],
            validation_criteria: vec!["validation_successful".to_string()],
            estimated_time_seconds: 180,
            is_critical: true,
        };

        // Add steps to plan
        plan.add_step(a01).unwrap();
        plan.add_step(a02).unwrap();
        plan.add_step(b01).unwrap();
        plan.add_step(b02).unwrap();
        plan.add_step(c).unwrap();
        plan.add_step(d).unwrap();

        plan
    }
}

impl SimulationStep {
    /// Get the phase of this step
    pub fn phase(&self) -> &str {
        &self.phase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_plan_creation() {
        let plan = SimulationPlan::new("Test Plan".to_string(), "A test plan".to_string());

        assert_eq!(plan.title, "Test Plan");
        assert_eq!(plan.description, "A test plan");
        assert!(plan.steps.is_empty());
        assert!(plan.dependencies.is_empty());
    }

    #[test]
    fn test_add_step() {
        let mut plan = SimulationPlan::new("Test Plan".to_string(), "A test plan".to_string());

        let step = SimulationStep {
            id: Uuid::new_v4(),
            phase: "A01".to_string(),
            title: "Test Step".to_string(),
            description: "A test step".to_string(),
            step_type: SimulationStepType::Analysis,
            inputs: vec![],
            outputs: vec![],
            validation_criteria: vec![],
            estimated_time_seconds: 30,
            is_critical: false,
        };

        assert!(plan.add_step(step).is_ok());
        assert_eq!(plan.steps.len(), 1);
    }

    #[test]
    fn test_invalid_step() {
        let mut plan = SimulationPlan::new("Test Plan".to_string(), "A test plan".to_string());

        let step = SimulationStep {
            id: Uuid::new_v4(),
            phase: "A01".to_string(),
            title: "".to_string(), // Empty title
            description: "A test step".to_string(),
            step_type: SimulationStepType::Analysis,
            inputs: vec![],
            outputs: vec![],
            validation_criteria: vec![],
            estimated_time_seconds: 30,
            is_critical: false,
        };

        assert!(plan.add_step(step).is_err());
    }

    #[test]
    fn test_steps_by_phase() {
        let plan = SimulationPlan::mock();

        let phase_a_steps = plan.steps_by_phase("A");
        assert_eq!(phase_a_steps.len(), 2);

        let phase_b_steps = plan.steps_by_phase("B");
        assert_eq!(phase_b_steps.len(), 2);

        let phase_c_steps = plan.steps_by_phase("C");
        assert_eq!(phase_c_steps.len(), 1);

        let phase_d_steps = plan.steps_by_phase("D");
        assert_eq!(phase_d_steps.len(), 1);
    }

    #[test]
    fn test_phases() {
        let plan = SimulationPlan::mock();
        let phases = plan.phases();

        assert!(phases.contains(&"A01".to_string()));
        assert!(phases.contains(&"A02".to_string()));
        assert!(phases.contains(&"B01".to_string()));
        assert!(phases.contains(&"B02".to_string()));
        assert!(phases.contains(&"C".to_string()));
        assert!(phases.contains(&"D".to_string()));
    }

    #[test]
    fn test_dependencies() {
        let mut plan = SimulationPlan::new("Test Plan".to_string(), "A test plan".to_string());

        let step1_id = Uuid::new_v4();
        let step2_id = Uuid::new_v4();

        plan.add_dependency(step2_id, step1_id);

        assert!(plan.dependencies.contains_key(&step2_id));
        assert_eq!(plan.dependencies.get(&step2_id).unwrap().len(), 1);
        assert_eq!(plan.dependencies.get(&step2_id).unwrap()[0], step1_id);
    }

    #[test]
    fn test_validation() {
        let plan = SimulationPlan::mock();
        let errors = plan.validate().unwrap();
        assert!(errors.is_empty(), "Mock plan should be valid: {:?}", errors);
    }

    #[test]
    fn test_invalid_plan() {
        let plan = SimulationPlan::new(
            "".to_string(), // Empty title
            "".to_string(), // Empty description
        );

        let errors = plan.validate().unwrap();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_circular_dependencies() {
        let mut plan = SimulationPlan::new("Test Plan".to_string(), "A test plan".to_string());

        let step1_id = Uuid::new_v4();
        let step2_id = Uuid::new_v4();

        // Create circular dependency
        plan.add_dependency(step1_id, step2_id);
        plan.add_dependency(step2_id, step1_id);

        assert!(plan.has_circular_dependencies());
    }
}
