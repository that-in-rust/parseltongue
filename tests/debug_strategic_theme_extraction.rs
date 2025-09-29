use parseltongue::content_processing::{
    ContentChunk, LineRange,
    strategic_theme_organizer::{
        StrategicInsightSynthesizer, StrategicThemeCategory,
    }
};

#[test]
fn debug_strategic_theme_extraction() {
    let synthesizer = StrategicInsightSynthesizer::new();
    
    let content = r#"
# Developer Productivity Enhancement

This strategic theme focuses on improving developer productivity through automation and tooling.

## Competitive Advantages
- Unique value proposition: 10x faster code analysis
- Market differentiation: Zero-configuration setup
- Technical moat: Advanced AST processing

## ROI Analysis
- Metric: Development time savings
- Baseline: 8 hours per week on code analysis
- Target: 1 hour per week
- Measurement methodology: Time tracking studies
- Timeframe: 6 months

## Adoption Strategy
- Target persona: Individual developers
- Stage 1: Tool installation and setup
- Stage 2: Integration with existing workflow
- Barriers: Learning curve, tool compatibility
- Enablers: Comprehensive documentation, tutorials
"#;
    
    let chunk = ContentChunk::new(
        content.to_string(),
        LineRange::new(1, 25),
        "test_advisory.md".to_string(),
    );
    
    println!("Content to analyze:\n{}", content);
    
    let themes = synthesizer.extract_strategic_themes(&chunk).unwrap();
    
    println!("Number of themes extracted: {}", themes.len());
    
    for (i, theme) in themes.iter().enumerate() {
        println!("Theme {}: {}", i + 1, theme.title);
        println!("  Category: {:?}", theme.category);
        println!("  Description: {}", theme.description);
        println!("  Competitive advantages: {}", theme.competitive_advantages.len());
        println!("  ROI metrics: {}", theme.roi_metrics.len());
        println!("  Adoption pathways: {}", theme.adoption_pathways.len());
        println!("  Confidence score: {}", theme.confidence_score);
    }
    
    // Test individual extraction methods
    let chunk_id = parseltongue::content_processing::ChunkId::new();
    let advantages = synthesizer.synthesize_competitive_advantages(content, chunk_id).unwrap();
    println!("Direct competitive advantages extraction: {}", advantages.len());
    
    let metrics = synthesizer.capture_roi_metrics(content, chunk_id).unwrap();
    println!("Direct ROI metrics extraction: {}", metrics.len());
    
    let pathways = synthesizer.map_adoption_pathways(content, chunk_id).unwrap();
    println!("Direct adoption pathways extraction: {}", pathways.len());
}