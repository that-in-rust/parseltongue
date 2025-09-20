

# **From Narrative to Algorithm: A Framework for Executable Specifications in AI-Driven Software Engineering**

## **The Inevitability of Rigor: Why Ambiguity is the Enemy of Automated Code Generation**

The advent of Large Language Models (LLMs) capable of generating complex software represents a watershed moment in software engineering. These models promise to automate the translation of human intent into functional code, accelerating development cycles and augmenting developer productivity.1 However, this promise is predicated on a fundamental challenge: the quality of the generated code is inextricably linked to the quality of the input specification. The prevalent methodologies for defining software requirements, honed over decades for human collaboration, are proving to be fundamentally incompatible with the formal, literal nature of AI systems. This analysis posits that a paradigm shift is necessary—away from descriptive, narrative specifications and towards rigorous, executable blueprints.

### **The Paradox of Agile Requirements in the Age of AI**

Modern software development is dominated by Agile methodologies, with the User Story as the primary unit for capturing requirements. A user story is intentionally lightweight—an "informal, general explanation of a software feature" 2 that serves as a "promise to have a conversation about requirements".3 Its format, "As a

, I need \<what?\>, so that \<why?\>" 4, is designed to build empathy, provide context, and foster collaboration among team members.6 Crucially, user stories are "negotiable" by design; they are placeholders for features that the team will discuss and clarify closer to the time of development.4

This intentional ambiguity is a powerful feature for human teams. It allows for emergent design and flexibility, acknowledging that it is often counterproductive to define all details at the outset of a project.4 The "real" specification materializes from the high-bandwidth conversation that follows the story's creation, a dialogue rich with shared context, iterative clarification, and implicit understanding.3

For an LLM, however, this ambiguity is a critical flaw. An LLM operates on an explicit, low-bandwidth communication channel—the text of its prompt. It cannot participate in a "conversation" to clarify unstated assumptions or negotiate details. When presented with an ambiguous user story, the model is forced to interpret, infer, and ultimately guess. This leads to common failure modes: architectural deviations, logical errors, security vulnerabilities, and code that, while syntactically correct, fails to meet the true business need. The success of Agile methodologies inadvertently created a dependency on implicit human communication, and the proposed shift to executable specifications directly addresses this "bandwidth mismatch." To achieve high-fidelity output from an LLM, the input specification must be pre-loaded with all the information that would normally be exchanged in the human conversation, making it dense, explicit, and unambiguous.

### **From Waterfall to Executable: A Necessary Synthesis**

The evolution of software requirements can be seen as a pendulum swing. The Waterfall model relied on comprehensive, rigid, and detailed functional specification documents that attempted to capture every requirement upfront.3 While rigorous, this approach was often brittle, slow, and unable to adapt to changing business needs. In response, Agile methodologies swung the pendulum towards the conversational, flexible user story.2

The concept of an executable specification represents a synthesis of these two extremes. It retains the rigor, completeness, and low ambiguity of Waterfall-era specifications but embeds them within a dynamic, verifiable, and iterative framework characteristic of Agile. The specification is no longer a static document to be filed away; it becomes a living, executable project skeleton that directly guides and verifies the code generation process.

| Paradigm | Primary Artifact | Primary Audience | Form | Tolerance for Ambiguity | Locus of Interpretation | Suitability for LLMs |
| :---- | :---- | :---- | :---- | :---- | :---- | :---- |
| **Waterfall Specification** | Formal Document (e.g., SRS) | Business Analysts & Architects | Narrative Prose | Very Low (in theory) | Human Developer | Low |
| **Agile User Story** | Index Card / Ticket | Cross-functional Team | Conversational Prompt | Very High (by design) | Collaborative Conversation | Very Low |
| **Executable Specification** | Code & Test Suite | Human Specifier & LLM Translator | Formal Logic (Code) | Zero | Verification Harness | High |
|  |  |  |  |  |  |  |

### **Defining the "Executable Specification"**

An Executable Specification is a suite of artifacts, written in a formal language (such as code, data definition language, and machine-readable diagrams), that defines a system's architecture, logic, and correctness criteria with sufficient rigor to eliminate ambiguity and serve as a direct, verifiable blueprint for automated code generation. Within this paradigm, the role of the LLM is transformed. It is no longer an *interpreter* of vague requirements but a *translator* of a precise, algorithmic blueprint into a target programming language. This approach provides the necessary human oversight *a priori*, embedding correctness checks and architectural constraints directly into the source prompt and aligning with the observation that AI-generated code requires diligent review and optimization.1

## **The Intellectual Antecedents: Grounding Executable Specifications in Computer Science**

The proposed framework, while novel in its application to LLMs, is not an ad-hoc invention. It stands on the shoulders of giants, representing a pragmatic and modern implementation of several foundational disciplines within computer science. By grounding the methodology in established theory, it transitions from a "prompt engineering strategy" to a robust and defensible engineering discipline.

### **A Modern Manifestation of Design by Contract (DbC)**

The primary theoretical pillar supporting this framework is Design by Contract (DbC), a methodology pioneered by Bertrand Meyer.9 DbC posits that software components should collaborate based on formal "contracts" that define mutual obligations and benefits between a "client" (the caller) and a "supplier" (the called method).10 A contract is composed of three key elements:

* **Preconditions:** These are the obligations of the client. They are conditions that must be true *before* a method is invoked. The client is responsible for satisfying them.9 In the provided  
  MessageService example, the function signature with its specific types (String, RoomId, UserId, Uuid) and the documented constraint (// Invariant: 1-10000 chars, sanitized HTML) constitute the preconditions.  
* **Postconditions:** These are the obligations of the supplier. They are conditions that the method guarantees will be true *after* it executes successfully, provided the preconditions were met.13 The documented  
  Side Effects (e.g., "Inserts row into 'messages'") and the precise return type Result\<Message\<Persisted\>, MessageError\> are the explicit postconditions.  
* **Invariants:** These are conditions that must hold true for an object or system throughout its lifecycle, maintained across method calls.10 The proposed L1  
  constraints.md file, which defines system-wide rules, serves as the specification for these global invariants.

The most significant innovation of the executable specification framework is how it enforces these contracts. While classic DbC often relies on language-level assertions checked at runtime 11, this methodology introduces the

RED phase: the provision of a complete, executable, and initially failing test suite. This transforms the contract from mere documentation into the very definition of correctness. The specification does not just *state* the postconditions; it provides the precise, automated *mechanism to verify* their fulfillment. The LLM's task is thus narrowed from the abstract goal of "implement the contract" to the concrete, verifiable objective of "make this specific test suite pass."

### **Pragmatic Formalism: Applying Formal Methods without the Overhead**

Formal Methods are a class of mathematically rigorous techniques used for the specification, design, and verification of software and hardware systems.19 By modeling a system as a mathematical entity, it becomes possible to prove its properties with a level of certainty that empirical testing cannot achieve.22 However, traditional formal methods often require specialized and complex notations (e.g., Z notation, B-Method) and significant expertise, limiting their widespread adoption.23

The executable specification framework can be understood as a form of *pragmatic* or *lightweight formal methods*. It eschews a separate mathematical language and instead leverages the formalisms inherent in modern, strongly-typed programming languages. The L2 architecture.md, with its complete SQL DDL and exhaustive Rust error enum definitions, is a formal specification of the system's state space and failure modes. The L3 STUB interface, with its precise types and traits, is a formal specification of a component's behavior. The RED test suite, particularly with the inclusion of property tests that check for abstract invariants, serves as the formal verification harness.22 This approach democratizes formal specification, making its benefits of clarity, rigor, and early error detection accessible to any development team.

### **TDD as a Machine-Readable Protocol: The Right Tool for the Job**

The choice of Test-Driven Development (TDD) as the core methodology for L3 specifications is a deliberate and critical design decision. To understand its significance, one must contrast it with Behavior-Driven Development (BDD).

BDD is an extension of TDD that emphasizes collaboration between technical and non-technical stakeholders.25 It uses a natural, accessible language (e.g., Gherkin's

Given-When-Then syntax) to define system behavior from the user's perspective, ensuring that development aligns with business outcomes.27 BDD is optimized for human-to-human shared understanding.

TDD, conversely, is a developer-centric practice focused on defining the precise, testable behavior of a single unit of code.26 Its Red-Green-Refactor cycle is a rigorous, algorithmic process for building software components.

Communicating with an LLM is not a stakeholder collaboration problem; it is a formal instruction problem. The LLM is not a "non-technical team member" that needs to understand business intent through natural language. It is a formal code synthesizer that requires unambiguous, machine-readable instructions. The natural language of BDD would reintroduce the very ambiguity the framework seeks to eliminate. The precise, code-based nature of a TDD test suite, however, serves as the ideal communication protocol. It provides the LLM with a perfect, unambiguous definition of expected inputs, outputs, and side effects, making TDD the correct tool for this specific "audience."

| Executable Spec Component | Corresponding Principle | Purpose in the Framework |
| :---- | :---- | :---- |
| **L1 constraints.md** | System-Wide Invariants (DbC) | Defines global operational boundaries and non-functional requirements. |
| **L2 architecture.md (Data Models)** | Formal Specification (Data) | Provides an unambiguous blueprint for system state. |
| **L2 architecture.md (Error Hierarchy)** | Design by Contract (Failure Postconditions) | Guarantees robust handling of all known failure modes. |
| **L3 STUB (Interface Contract)** | Design by Contract (Pre/Postconditions) | Defines the precise obligations and guarantees of a single function. |
| **L3 RED (Failing Tests)** | Formal Verification / TDD | Provides an executable, verifiable oracle for functional correctness. |
| **L3 GREEN (Decision Tables)** | Algorithmic Specification | Eliminates logical ambiguity in complex business rules. |
| **L4 user\_journeys.md** | BDD / Acceptance Testing | Ensures technically correct modules combine to deliver end-user value. |
|  |  |  |

## **Anatomy of an Executable Blueprint: A Deep Analysis of the L1-L4 Framework**

The proposed four-layer structure provides a comprehensive, modular approach to specification that respects the context limitations of LLMs while ensuring complete coverage from high-level architecture to low-level logic.

### **L1: constraints.md – System-Wide Invariants and the Architectural Rulebook**

This foundational layer acts as the global contract for the entire system, codifying the architectural philosophy and non-functional requirements that transcend any single module. By defining constraints such as complexity limits (e.g., max lines per file), allowed libraries, and forbidden patterns, it establishes the boundaries within which the LLM must operate, ensuring consistency and maintainability.

To enrich this layer, specifications for logging standards, observability, and security should be included. For example, a constraint could mandate that all public-facing API endpoints must be processed by a specific authorization middleware, or that key business transactions must emit structured logs with certain correlation IDs. This elevates non-functional requirements to first-class, verifiable citizens of the specification.

### **L2: architecture.md – Architectural Contracts and the Principle of Fail-Fast**

This layer defines the static skeleton of the system: its components, their relationships, and the data structures they exchange. The use of Mermaid diagrams for architecture, complete Data Definition Language (DDL) for data models, and language-specific type definitions (e.g., Rust structs) provides an unambiguous structural blueprint.

The proposal's inclusion of an **Exhaustive Error Hierarchy** is a particularly powerful concept. By pre-defining a complete enum of every possible error a service can return, the specifier forces a "compile-time" consideration of all failure modes. This is a direct application of DbC's "fail hard" philosophy 10, preventing the LLM from inventing novel, unhandled error types and ensuring that error handling is a deliberate architectural concern, not an implementation afterthought.

This layer can be further strengthened by including formal API contracts using a standard like OpenAPI 3.0. This would not only provide a rigorous definition for RESTful interfaces but also enable the automatic generation of client libraries, mock servers, and API documentation, further enhancing the executable and verifiable nature of the specification.

### **L3: modules/\*.md – Method-Level Contracts and the TDD Cycle**

This is the core of the framework, where the system's dynamic logic is specified with precision. The TDD cycle provides a robust structure for this:

* **STUB (Interface Contract):** As established, this is the DbC contract, defining the method's signature, inputs, outputs, and documented side effects.  
* **RED (Behavioral Specification):** Providing executable, failing tests is the key innovation that makes the contract verifiable. The inclusion of **Property Tests** alongside unit tests is a mark of deep insight. While unit tests verify behavior against known, specific examples, property tests define and check abstract invariants (e.g., "for any valid input, idempotency holds"). This allows the machine to search for and identify edge cases and logical flaws that a human specifier might never anticipate.  
* **GREEN (Implementation Guidance):** The use of **Decision Tables** for complex conditional logic is vastly superior to prose or pseudocode. A decision table is a structured, tabular format that exhaustively maps combinations of conditions to specific actions and outputs. This format translates directly and unambiguously into code constructs like match statements or nested conditionals, leaving no room for misinterpretation by the LLM.  
* **REFACTOR (Constraints & Anti-Patterns):** This section acts as a set of crucial guardrails. Explicitly forbidding incorrect implementation strategies—such as prohibiting an application-level SELECT before INSERT to avoid Time-of-Check to Time-of-Use (TOCTOU) race conditions—closes off entire categories of potential bugs and architectural errors that an LLM might otherwise generate.

### **L4: user\_journeys.md – Behavioral Confirmation and End-to-End Validation**

This layer serves as the final bridge between technical correctness and business value. It ensures that the individual modules, proven correct by the L3 specifications, integrate properly to fulfill end-to-end user scenarios. This serves a similar purpose to BDD, confirming that the system's behavior meets high-level user expectations and acceptance criteria.27 The E2E test stubs provide a clear definition of done for the system as a whole. To maximize automation, these stubs should be structured in a way that allows for direct translation into a concrete testing framework like Playwright or Cypress, potentially by another specialized tool or LLM.

### **The Verification Harness (verification.md): Codifying the Definition of Flawless**

This final component is the lynchpin of the entire methodology. It operationalizes the definition of "flawless" by creating a single, executable script that runs all verification steps: static analysis, unit tests, property tests, integration tests, and E2E tests. This removes all subjectivity from the code review and acceptance process. The implementation is deemed correct if, and only if, the verification harness executes and exits with a status code of 0\. It is the ultimate expression of a truly executable and verifiable specification.

## **The New Division of Labor: The Software Architect as Specifier-in-Chief**

Adopting this methodology has profound implications that extend beyond technical execution, fundamentally reshaping the role of the software engineer and the structure of development teams.

### **From Coder to Architect: The Shift in Core Competencies**

When an LLM is responsible for the mechanical act of translating a detailed blueprint into a target language 1, the primary value-add of the human engineer shifts away from mastery of language syntax or rote algorithmic implementation. The most critical and valuable skills become those required to create the blueprint itself:

* **Systems Thinking:** The ability to design the coherent, scalable, and resilient high-level architectures defined in L1 and L2.  
* **Formal Logic:** The skill of translating ambiguous business requirements into the precise, unambiguous contracts, decision tables, property tests, and error hierarchies of an L3 specification.  
* **Adversarial Thinking:** The capacity to anticipate edge cases, failure modes, race conditions, and security vulnerabilities, and then to codify them as explicit RED tests that the implementation must guard against.  
* **Economic Thinking:** The wisdom to balance ideal correctness with practical constraints, formally defining areas of "acceptable imperfection" to deliver value efficiently without over-engineering.

### **Prompt Engineering is Not Enough: The Fallacy of the "AI Code Monkey"**

A simplistic view of AI's role in development is that engineers will simply write better natural language prompts to an "AI code monkey." This view is insufficient. Effective use of code generation tools requires a deep understanding of how to structure prompts for correctness and how to debug the resulting output.8

The Executable Specification framework represents the apotheosis of prompt engineering. The "prompt" is not a paragraph of English text; it is a multi-file, logically consistent, and self-verifying artifact. The act of creating this comprehensive prompt is not a trivial task but rather the core activity of a highly skilled software architect. It requires a profound understanding of the problem domain, software design principles, and formal verification techniques.

### **Team Structures and the "Specifier/Translator" Model**

This shift in responsibilities may lead to new team structures. A potential model is a division of labor between two key roles:

* **Architect-Specifiers:** Senior engineers who are experts in domain modeling, systems design, formal specification, and security. Their primary output is the L1-L4 executable blueprint. They are the authors of the system's logic and constraints.  
* **AI-Assisted Implementers/Verifiers:** Engineers who operate the LLM "translator" to generate code from the specifications. Their focus is on managing the generation pipeline, running the verification harness, debugging integration issues, and overseeing the deployment process. They ensure the smooth functioning of the automated translation and verification workflow.

## **Strategic Implementation and Future Directions**

The transition to a methodology of this rigor requires a deliberate strategy and will be accelerated by the development of a new ecosystem of tools.

### **An Incremental Adoption Pathway**

A "big bang" adoption of this entire framework is likely unrealistic and disruptive. A more pragmatic, incremental pathway is recommended:

1. **Start with L3:** For a single, new, well-encapsulated module, apply the rigorous STUB \-\> RED \-\> GREEN \-\> REFACTOR cycle. This introduces the core discipline of contract-based, test-driven specification at a manageable scale.  
2. **Introduce L2:** For the next new service, begin by authoring the architecture.md file, defining the data models and the exhaustive error hierarchy *before* writing the L3 module specifications.  
3. **Formalize L1 and L4:** As the culture of rigor takes hold, formalize the L1 system-wide constraints and begin building out the L4 E2E test stubs for major user journeys.  
4. **Integrate the Verification Harness:** Finally, connect all the pieces by creating the master verification.md script, fully automating the definition of "done."

### **Tooling and Ecosystem Requirements**

This methodology would be significantly enhanced by a new generation of Integrated Development Environments (IDEs) designed for specification-first development. One can envision an IDE where defining an L3 STUB interface automatically generates the boilerplate for the RED test files. It might feature a graphical UI for creating Decision Tables that generates the corresponding markdown in the GREEN section. In such an IDE, the "Run" button would not merely compile code; it would trigger the entire pipeline of LLM generation, code compilation, and execution of the full verification harness.

### **Deconstructing "Rails-Equivalent Imperfection"**

The concept of formally specifying acceptable imperfections is one of the most sophisticated and pragmatic aspects of this framework. Formal methods traditionally aim to prove absolute correctness 19, a standard that is often unnecessary or cost-prohibitive for many real-world system properties. Business requirements frequently involve trade-offs that embrace non-absolute behaviors like eventual consistency, statistical reliability, or defined performance tolerances.

By providing a test that asserts a state *after* a delay (e.g., the 65-second window for presence tracking), the specification codifies the business requirement ("it's acceptable for presence to be slightly out of date") into a verifiable, executable contract. This methodology allows a team to apply the full rigor of formal specification not only to absolute correctness properties but also to these nuanced, non-ideal-but-acceptable behaviors. This represents a significant evolution, bridging the gap between the absolute world of formal verification and the pragmatic, resource-constrained world of business value.

### **Beyond Code Generation: The Specification as a Central Asset**

An executable specification suite is far more than a one-time prompt for an LLM. It is a valuable, long-lived engineering asset that serves multiple purposes:

* **Automated Documentation:** The L2 diagrams, DDL, and L3 contracts can be used to generate perfectly accurate, always-up-to-date technical documentation, solving a perennial problem in software maintenance.  
* **Enhanced Security Audits:** Formal models of system behavior and data flow allow security analysis tools to identify potential vulnerabilities at the specification stage, before a single line of implementation code is written.  
* **Intelligent Maintenance:** When a requirement changes, the engineer's first step is to modify the specification (e.g., by adding a new RED test or updating a decision table). The LLM can then be tasked with refactoring the code to meet the new specification, and the verification harness automatically ensures that the change is implemented correctly and introduces no regressions.

## **Conclusion: A Paradigm Shift Towards Correct-by-Construction Software**

The proposed framework for executable specifications is not merely an incremental improvement in prompting techniques. It is a coherent, robust, and theoretically sound methodology that addresses the fundamental challenge of ambiguity in AI-driven software development. It correctly diagnoses the shortcomings of traditional requirements and provides a comprehensive solution grounded in decades of research in Design by Contract, formal methods, and test-driven development.

Far from deskilling the software engineering profession, the rise of capable LLMs is acting as a powerful forcing function. It is compelling the industry to finally adopt the level of precision, rigor, and formal discipline that has long been advocated by computer science theorists but often overlooked in practice. The need to communicate with a literal-minded machine is making the benefits of formal specification undeniably clear.

The ultimate promise of this methodology is a paradigm shift towards **correct-by-construction software**. When the specification is a formal, executable blueprint and the verification process is comprehensive and automated, the code generated is not just "probably right"—it is provably correct with respect to that specification. This represents a profound step forward in our ability to engineer reliable, robust, and secure software systems.

#### **Works cited**

1. Is There a Future for Software Engineers? The Impact of AI \[2025\] \- Brainhub, accessed on September 15, 2025, [https://brainhub.eu/library/software-developer-age-of-ai](https://brainhub.eu/library/software-developer-age-of-ai)  
2. User Stories | Examples and Template \- Atlassian, accessed on September 15, 2025, [https://www.atlassian.com/agile/project-management/user-stories](https://www.atlassian.com/agile/project-management/user-stories)  
3. User stories vs Functional specifications \- Project Management Stack Exchange, accessed on September 15, 2025, [https://pm.stackexchange.com/questions/20948/user-stories-vs-functional-specifications](https://pm.stackexchange.com/questions/20948/user-stories-vs-functional-specifications)  
4. Agile Requirements and User Stories \- what is the difference?, accessed on September 15, 2025, [https://www.agilebusiness.org/dsdm-project-framework/requirements-and-user-stories.html](https://www.agilebusiness.org/dsdm-project-framework/requirements-and-user-stories.html)  
5. How To Capture the Technical Details With User Stories \- 3Pillar Global, accessed on September 15, 2025, [https://www.3pillarglobal.com/insights/blog/how-to-capture-the-technical-details-with-user-stories/](https://www.3pillarglobal.com/insights/blog/how-to-capture-the-technical-details-with-user-stories/)  
6. Requirements vs User Stories vs Acceptance Criteria : r/agile \- Reddit, accessed on September 15, 2025, [https://www.reddit.com/r/agile/comments/123k627/requirements\_vs\_user\_stories\_vs\_acceptance/](https://www.reddit.com/r/agile/comments/123k627/requirements_vs_user_stories_vs_acceptance/)  
7. User Story vs Requirement \- Software Engineering Stack Exchange, accessed on September 15, 2025, [https://softwareengineering.stackexchange.com/questions/212834/user-story-vs-requirement](https://softwareengineering.stackexchange.com/questions/212834/user-story-vs-requirement)  
8. Why AI is making software dev skills more valuable, not less : r/ChatGPTCoding \- Reddit, accessed on September 15, 2025, [https://www.reddit.com/r/ChatGPTCoding/comments/1h6qyl0/why\_ai\_is\_making\_software\_dev\_skills\_more/](https://www.reddit.com/r/ChatGPTCoding/comments/1h6qyl0/why_ai_is_making_software_dev_skills_more/)  
9. Design by Contract \- PKC \- Obsidian Publish, accessed on September 15, 2025, [https://publish.obsidian.md/pkc/Hub/Tech/Design+by+Contract](https://publish.obsidian.md/pkc/Hub/Tech/Design+by+Contract)  
10. Design by contract \- Wikipedia, accessed on September 15, 2025, [https://en.wikipedia.org/wiki/Design\_by\_contract](https://en.wikipedia.org/wiki/Design_by_contract)  
11. Design by Contract for Embedded Software \- Quantum Leaps, accessed on September 15, 2025, [https://www.state-machine.com/dbc](https://www.state-machine.com/dbc)  
12. Design by Contract: How can this approach help us build more robust software?, accessed on September 15, 2025, [https://thepragmaticengineer.hashnode.dev/design-by-contract-how-can-this-approach-help-us-build-more-robust-software](https://thepragmaticengineer.hashnode.dev/design-by-contract-how-can-this-approach-help-us-build-more-robust-software)  
13. Design by Contract Introduction \- Eiffel Software \- The Home of EiffelStudio, accessed on September 15, 2025, [https://www.eiffel.com/values/design-by-contract/introduction/](https://www.eiffel.com/values/design-by-contract/introduction/)  
14. Design by contracts \- learn.adacore.com, accessed on September 15, 2025, [https://learn.adacore.com/courses/intro-to-ada/chapters/contracts.html](https://learn.adacore.com/courses/intro-to-ada/chapters/contracts.html)  
15. Design by Contract for statecharts — Sismic 1.6.10 documentation \- Read the Docs, accessed on September 15, 2025, [https://sismic.readthedocs.io/en/latest/contract.html](https://sismic.readthedocs.io/en/latest/contract.html)  
16. Danish University Colleges Contract-Based Software Development: Class Design by Contract Kongshøj, Simon \- UC Viden, accessed on September 15, 2025, [https://www.ucviden.dk/files/124401058/dbc01.pdf](https://www.ucviden.dk/files/124401058/dbc01.pdf)  
17. What is Design by Contract? \- Educative.io, accessed on September 15, 2025, [https://www.educative.io/answers/what-is-design-by-contract](https://www.educative.io/answers/what-is-design-by-contract)  
18. Design by Contract \- SAP Community, accessed on September 15, 2025, [https://community.sap.com/t5/additional-blogs-by-sap/design-by-contract/ba-p/12845892](https://community.sap.com/t5/additional-blogs-by-sap/design-by-contract/ba-p/12845892)  
19. What is Formal Methods, accessed on September 15, 2025, [https://shemesh.larc.nasa.gov/fm/fm-what.html](https://shemesh.larc.nasa.gov/fm/fm-what.html)  
20. What Are Formal Methods? | Galois, accessed on September 15, 2025, [https://www.galois.com/what-are-formal-methods](https://www.galois.com/what-are-formal-methods)  
21. Formal methods \- Wikipedia, accessed on September 15, 2025, [https://en.wikipedia.org/wiki/Formal\_methods](https://en.wikipedia.org/wiki/Formal_methods)  
22. Formal Methods \- Carnegie Mellon University, accessed on September 15, 2025, [https://users.ece.cmu.edu/\~koopman/des\_s99/formal\_methods/](https://users.ece.cmu.edu/~koopman/des_s99/formal_methods/)  
23. Safe by Design: Examples of Formal Methods in Software Engineering \- SoftwareHut, accessed on September 15, 2025, [https://softwarehut.com/blog/tech/examples-of-formal-methods](https://softwarehut.com/blog/tech/examples-of-formal-methods)  
24. Formal methods in software engineering \- Educative.io, accessed on September 15, 2025, [https://www.educative.io/answers/formal-methods-in-software-engineering](https://www.educative.io/answers/formal-methods-in-software-engineering)  
25. TDD vs. BDD: What's the Difference? (Complete Comparison ..., accessed on September 15, 2025, [https://semaphore.io/blog/tdd-vs-bdd](https://semaphore.io/blog/tdd-vs-bdd)  
26. TDD vs. BDD: What's the Difference? \- Ranorex, accessed on September 15, 2025, [https://www.ranorex.com/blog/tdd-vs-bdd/](https://www.ranorex.com/blog/tdd-vs-bdd/)  
27. TDD VS BDD: Detailed Comparison \- TestGrid, accessed on September 15, 2025, [https://testgrid.io/blog/tdd-vs-bdd-which-is-better/](https://testgrid.io/blog/tdd-vs-bdd-which-is-better/)  
28. Understanding the differences between BDD & TDD \- Cucumber, accessed on September 15, 2025, [https://cucumber.io/blog/bdd/bdd-vs-tdd/](https://cucumber.io/blog/bdd/bdd-vs-tdd/)  
29. TDD vs BDD vs ATDD : Key Differences \- BrowserStack, accessed on September 15, 2025, [https://www.browserstack.com/guide/tdd-vs-bdd-vs-atdd](https://www.browserstack.com/guide/tdd-vs-bdd-vs-atdd)  
30. TDD vs BDD: Your Pocket Cheat-Sheet \- Testim, accessed on September 15, 2025, [https://www.testim.io/blog/tdd-vs-bdd-a-developers-pocket-reference-with-examples/](https://www.testim.io/blog/tdd-vs-bdd-a-developers-pocket-reference-with-examples/)