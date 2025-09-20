# Proposal: Enhancing Documentation for TDD and Feature Specifications

## Overview of Documentation Improvements

To fully embrace **type-driven, test-first development**, we propose updating the Campfire documentation to make each Markdown spec a *living template* that captures design **and** testing plans **before** coding. Currently, the docs already emphasize writing interfaces first (e.g. defining complete type contracts and function signatures up front[\[1\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=This%20document%20defines%20the%20complete,first%20success%20and%20architectural%20correctness)), and acceptance criteria are written in a BDD-style “WHEN … THEN … SHALL …” format[\[2\]](file://file-AgNPwiG9jBq57Aahvq66nU#:~:text=). Building on this foundation, we will extend each document to incorporate explicit test scaffolding and end-to-end feature validation. The goal is to minimize manual work by reusing structured templates and even leveraging LLMs to generate code and tests from the specs. Below we detail proposed changes to each document, followed by templates and checklists to ensure full feature coverage.

## Design Document (design.md) – Include Interfaces *and* Test Contracts

**What to add:** After each interface or module definition, embed a subsection outlining the **test contract** for that component. This includes the integration scenarios, preconditions, and expected outcomes (essentially a spec for the tests that will be written). The design doc already lists all domain types and trait method signatures in detail[\[3\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=MessageService%20)[\[4\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20,Updates%20FTS5%20search%20index). We will augment it with a “Test Plan” for each major trait or feature. For example:

* *After defining MessageService trait methods, add* *“\#\#\#\# MessageService Test Plan”*\*\*. This section will enumerate how each method is verified:

* **Scenario:** Create a new message with valid data → **Preconditions:** user is a room member; **Action:** call create\_message\_with\_deduplication; **Expected:** returns Ok(Message) and broadcast occurs (simulate via a stub WebSocket), etc.

* **Scenario:** Duplicate client\_message\_id → **Preconditions:** same UUID was used in a prior message; **Action:** call create\_message\_with\_deduplication again; **Expected:** returns existing message (no duplicate in DB).

* **Scenario:** Unauthorized user → **Precondition:** user not in room; **Action:** call create; **Expected:** Err(MessageError::Authorization).

* *Repeat for other services:* e.g. **“\#\#\#\# RoomService Test Plan”** covering cases like creating open vs. closed rooms, duplicate direct room prevention, etc.

Each scenario in these test plans will clearly state **preconditions, the action, and postconditions** (outcome), effectively turning the design spec into an executable test blueprint. This makes the design doc a one-stop reference for both the interface and how to verify it. It aligns with the “integration test contracts” step currently outlined as a next step in the design summary[\[5\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2A%2ANext%20Steps%3A%2A%2A%201.%20Create%20property,guided%20implementation%20following%20these%20contracts), but now we’ll bake those contracts directly into the design spec.

**Why:** This ensures that for every function or type in the design, we have a corresponding test expectation before implementation. It reinforces the TDD workflow (types first, then tests, then code). It also provides the LLM with examples of how tests are structured for each interface. By including these structured scenarios (in list or table form) in the design doc, an LLM can more easily translate them into actual test code later.

**Example Update (Design.md excerpt):**

\#\#\# MessageService – Complete Interface Contract   
\*(existing trait definition here)\*

\#\#\#\# MessageService Test Plan    
\- \*\*Scenario 1: Successful Message Creation\*\*    
  \*\*Given\*\* a valid user in room and valid content, \*\*when\*\* \`create\_message\_with\_deduplication\` is called, \*\*then\*\* it returns \`Ok(Message\<Persisted\>)\` and the new message is saved, \`room.last\_message\_at\` is updated, and a WebSocket broadcast is triggered.  

\- \*\*Scenario 2: Deduplication of Message\*\*    
  \*\*Given\*\* a message with \`client\_message\_id\` X already exists, \*\*when\*\* a new message with the same client ID X is created in that room, \*\*then\*\* the service returns \`Ok(existing Message)\` (no duplicate) – fulfilling Critical Gap \#1 deduplication\[6\]\[7\].  

\- \*\*Scenario 3: Unauthorized Creator\*\*    
  \*\*Given\*\* a user without access to the room, \*\*when\*\* they attempt to create a message, \*\*then\*\* the service returns \`Err(MessageError::Authorization)\` and no message is created.  

\*(and so on for update, delete, search scenarios)…\*

By writing the test plan in the design doc, we ensure **full behavioral coverage** is designed upfront, not left implicit. We will follow this pattern for every major feature or service in the design doc: first the type/trait definition, then the test scenarios covering all its behaviors (including edge cases, error cases, and side effects). This mirrors the **TDD-driven design** approach (define contract, then tests) within the documentation itself.

## Requirements Document (requirements.md) – Tie Acceptance Criteria to Tests

The requirements doc already lists each feature’s user story and detailed acceptance criteria in a structured, Kiro-style format. We will enhance this by making each acceptance criterion explicitly **testable** and traceable to the test plan. Specifically:

* **Maintain “WHEN…, THEN…, SHALL…” style:** This format will remain for clarity and consistency[\[2\]](file://file-AgNPwiG9jBq57Aahvq66nU#:~:text=). Each criterion describes an observable behavior which is essentially a test expectation. We will review each acceptance criterion to ensure it reads like a testable scenario with clear conditions and outcomes. For example, an acceptance from Requirement 1 might be refined as: *“WHEN a user sends a message with only emojis THEN the message SHALL be displayed with enlarged emoji styling.”* Such phrasing directly translates into an integration test scenario (simulate sending an emoji-only message and assert the rendered result). If any criteria are too high-level, we break them into specific cases so that each can map to at least one test.

* **Tag criteria with IDs and map to tests:** The requirements already enumerate each criterion (1.1, 1.2, etc.). We propose using these IDs as a lightweight traceability mechanism. For example, if **Requirement 1.9** says emoji-only messages are enlarged, then the corresponding test scenario in the design’s test plan or in the test code can reference “Req 1.9” in comments or naming. This mapping could even be included in the design/test plan sections (e.g., *Scenario X covers Requirement 1.9*). By doing so, we ensure **full coverage of user journeys**: every behavior the PM expects (from user stories) is accounted for in our planned tests. It will be easy to see if a requirement has no test (which would indicate a gap).

* **Example in requirements.md:** We might add a short **“Verification”** note after the acceptance criteria of each requirement section, summarizing how it will be verified. For instance:

\*\*Verification:\*\* Requirement 1.9 will be validated by an integration test simulating an emoji-only message post and checking that the frontend renders it with enlarged styling (see MessageService Test Plan, Scenario “Emoji-only enlargement”). 

This cross-references the design/test plan and makes the requirement document not just a statement of intent but also a pointer to its validation method.

In summary, updates to requirements.md will strengthen the **TDD linkage**: each acceptance criterion leads naturally into a test case. This gives both developers and PM confidence that “if the tests pass, the requirements are met.” It also aids LLMs – the clear, test-oriented wording can be used to prompt generation of test code (for example, feeding an acceptance criterion into an LLM to get a test function outline).

## Implementation Tasks (tasks.md) – Use Checklist and Templates for Test-First Execution

The tasks.md (implementation plan) will be updated to explicitly incorporate writing tests as part of the development tasks, and to serve as a **checklist for full feature completion**. Currently, tasks.md already includes TDD-oriented tasks (e.g. Phase 0 includes defining interfaces and writing property tests before implementation[\[8\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=0)). We will enrich this by:

* **Embedding test development steps per feature:** For each feature or phase, include subtasks for creating the test scaffolding. For example, if Phase 2 includes implementing MessageService, the subtasks should be:

* “Write integration tests for message flows (covering send, edit, delete, search) – *expected outcomes per design spec*.”

* “Implement MessageService methods to satisfy the above tests.” By listing the test writing first (with checkboxes), we reinforce writing tests before code. We already see this pattern in Phase 0 of tasks.md where property tests are outlined[\[8\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=0); we will apply it consistently for all features (not just core invariants, but user-level scenarios too).

* **Use checklist format for verification:** At the end of each feature section, include a brief **“Feature Verification Checklist.”** This is a set of items that must be true for the feature to be considered done, effectively summarizing the acceptance criteria and test results in checklist form. For example, for “Rich Text Message System” feature, the checklist might be:

* \[ \] User can format text with bold/italic and see it rendered correctly (Req 1.2)

* \[ \] /play sound commands play the correct sound (Req 1.5)

* \[ \] Emoji-only messages appear enlarged (Req 1.9)

* \[ \] All above behaviors covered by automated tests (all green)

* \[ \] No regression in core messaging flow (smoke test passes)

These checkboxes tie directly to both requirements and tests. Developers can tick them off as they implement and test each part, and reviewers can quickly see if anything was missed. Tasks.md already has a success metrics and known limitations section[\[9\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=Success%20Metrics)[\[10\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=Performance%20Requirements%20%28Rails,100MB) – the new feature checklists will complement those by focusing on **specific feature behavior verification** rather than high-level metrics.

* **Template tasks for new features:** We will create a generic tasks template that can be copied for any new feature addition. For example:

* \#\#\# Feature X Implementation Plan  
  \- \[ \] \*\*Design Phase\*\* – Define all new types, traits, and functions for \*Feature X\* (update design.md with stubs and docs)  
  \- \[ \] \*\*Test Phase\*\* – Write unit tests and integration tests for all \*Feature X\* scenarios (failing tests initially)  
  \- \[ \] \*\*Implementation Phase\*\* – Implement functionality to make tests pass, adhering to design contracts  
  \- \[ \] \*\*Documentation\*\* – Update requirements.md (acceptance criteria) and architecture.md (if needed) for \*Feature X\*, ensure all docs reflect the final design  
  \- \[ \] \*\*Verification\*\* – All \*Feature X\* tests pass; run full feature flow CLI/test to confirm (see below)

* This template ensures every new feature follows the same cycle: spec → test → code → docs, and is reflected in the tasks checklist. It minimizes effort by providing a ready-made structure. An LLM can be prompted with this template to fill in specific tasks for a given feature description, further automating planning.

By enhancing tasks.md in this way, we make it a practical **engineering workbook** that guides developers through TDD for each feature and tracks completeness. The clear ordering (tests before code) and mapping to requirements will maximize reuse of this pattern across features.

## Architecture & Testing (architecture.md) – One-Command Full Flow Verification

Our architecture document will be extended to describe the unified testing approach and any tooling to support the “single command” verification of full user flows. The current architecture.md briefly lists testing levels (unit, integration, end-to-end)[\[11\]](file://file-PW3T15a4YVXCsZ2Xe8esJ8#:~:text=,Verify%20behavior%20matches%20Rails%20ActionCable). We will build on this by specifying how to run a full end-to-end test scenario easily:

* **Define a CLI or test harness:** We propose a special integration test (or small binary) that orchestrates a complete user journey for a given feature. For example, a test that simulates: **“User signup → login → create room → post message → another user receives message via WebSocket.”** This would combine API calls and direct service calls to mimic a real workflow. By automating this in code, a developer (or PM, if technical) can run it with a single command. Concretely, this could be an integration test file (e.g. tests/feature\_full\_flow.rs) that uses the compiled server in-memory. Running cargo test \--test feature\_full\_flow would execute the scenario. We’ll note in architecture.md that **each major user journey has a corresponding automated scenario test**.

* **Document the “single command” usage:** In architecture.md’s testing strategy, add a note such as: *“For each feature, a one-click smoke test validates the entire user flow. For example, running cargo test \--features=rich\_text\_flow executes all steps of the Rich Text Messaging journey, ensuring that from frontend API to backend DB and WebSocket, everything works as expected.”* We can provide instructions for how to run these and interpret results. If we implement a separate CLI tool (say, a dev-only command within the app), we’ll document campfire-on-rust \--smoke-test featureX as another option. The key is to advertise that verifying a feature is as simple as running one target, rather than manually clicking through the UI.

* **Continuous integration hook:** We will also mention that these end-to-end tests are part of CI, so every push runs the full feature flows automatically. This guarantees that any regression in a user journey is caught immediately. Architecture.md can include a section about automated testing pipeline: e.g., “**CI Automation:** All unit, integration, and end-to-end tests (including full feature scenarios) are run on each build. This serves as an automated QA verifying all documented behaviors.”

By including this in the architecture doc, we formalize the practice of full-flow testing. It complements the earlier docs: requirements define *what* should happen, design defines *how* at a high level, and now architecture/testing defines *how to verify it continuously*. It also underscores our **pragmatic testing focus** – not aiming for complex frameworks, just simple commands to ensure reliability (consistent with our simplicity mandate).

## Reusable Feature Spec Template for LLMs

To maximize reuse and minimize manual effort, we will treat the structure of these MD documents as a **template for specifying new features**. This template can be given to an LLM along with feature details to generate initial stubs and tests conforming to our conventions. The template (in Markdown form) would look like:

\# Feature: \<Feature Name\>

\#\# User Story    
As a \<user role\>, I want \<feature goal\> so that \<benefit\>.

\#\# Acceptance Criteria    
1\. WHEN \<context or action\> THEN it SHALL \<expected behavior\>.    
2\. WHEN \<another scenario\> THEN it SHALL \<expected outcome\>.    
\*(Include all key behaviors and edge cases in numbered criteria.)\*

\#\# Design    
\*\*Data Model/Types:\*\* List any new structs or fields required, with descriptions.    
\*\*Service Interfaces:\*\* Outline new trait methods or functions (with signatures) needed to implement this feature. Use Rust syntax and include doc comments for each, specifying inputs, outputs, errors (following the style in design.md\[12\]\[13\]).    
\*\*Error Handling:\*\* Define any new error types or variants to cover failure cases.  

\#\# Test Plan    
\*\*Unit Tests:\*\* Identify any pure functions or components to be unit-tested (if any).    
\*\*Integration Tests:\*\* Define scenarios with preconditions and expected results for end-to-end flows:  
\- \*Scenario 1:\* \*\*Precondition:\*\* \<state\> \*\*Action:\*\* \<user performs X\> \*\*Expected:\*\* \<outcome\> (maps to AC \#1)    
\- \*Scenario 2:\* ... (cover each Acceptance Criterion with at least one scenario)    
Include any special \*\*test fixtures\*\* (e.g., “requires a test user with admin role”, or “simulate network drop for reconnection test”).  

\#\# Tasks    
\- \[ \] \*\*Design\*\*: Update design.md with new types and interface stubs for \*Feature Name\*.    
\- \[ \] \*\*Tests\*\*: Write failing tests for each scenario above (in code or pseudocode form).    
\- \[ \] \*\*Implementation\*\*: Write code to make all tests pass, using TDD iteration.    
\- \[ \] \*\*Docs\*\*: Update documentation (requirements.md, architecture.md) to reflect implemented behavior and any constraints.    
\- \[ \] \*\*Verify\*\*: Run \`cargo test \--test \<feature\_flow\>\` – all scenarios passing.

When a developer or PM is kicking off a new feature, they can fill in this template. It ensures nothing is missed: from user story down to how we’ll verify it. The filled template can then be merged into the main docs (e.g., appending to requirements.md and design.md) or kept as a feature-specific spec during development. This template is also an excellent prompt for an LLM. For instance, we could feed the LLM the **Design** and **Test Plan** sections to generate the actual Rust trait code and Rust test code respectively. Because the template enforces our project’s conventions (like doc-comment style, error enums, Given/When/Then wording in tests), the LLM’s output will more likely conform to what we expect.

**Example Usage:** Suppose we want to add a “Message Pinning” feature. We draft the spec using the template. Then: \- Use the **Service Interfaces** section as input to have the LLM generate a trait implementation or function stubs (it sees how prior trait methods are formatted from design.md and mimics that). \- Use the **Test Plan** scenarios as input to generate Rust tests (the LLM sees the Given/When/Then structure and translates that into test functions using our testing framework, perhaps using our existing tests as examples).

The MD docs thus serve as both human-readable design and as machine-readable prompts. They effectively become **literate programming artifacts** where documentation and code generation blend. This drastically reduces manual coding of boilerplate since an LLM can take over repetitive patterns (for example, generating similar error handling in each method, consistent test structure, etc.).

## Full Feature Verification Checklist

As a final piece, we propose adopting a standardized **Feature Verification Checklist** to use during code reviews or before merging a feature branch. This checklist distills everything above into a yes/no list to ensure nothing falls through the cracks. It can be included in the pull request template or in tasks.md for each feature. Here’s a general checklist format:

* \[ \] **Design Spec Completed:** All new interfaces, types, and errors for the feature are defined in design.md (or feature spec) *before* implementation begins.

* \[ \] **Acceptance Criteria Met:** Every acceptance criterion from requirements.md for this feature has at least one corresponding test scenario implemented. *(Trace each criterion ID to a test result.)*

* \[ \] **All Tests Passing:** Unit tests, integration tests, and full user flow tests for the feature are written and all pass (cargo test is green).

* \[ \] **One-Command Flow OK:** Running the one-command smoke test for this feature (e.g. cargo test \--test feature\_x\_flow or equivalent) completes successfully, demonstrating the end-to-end user journey.

* \[ \] **Docs Updated:** Requirements, design, and architecture docs are updated to reflect the final implemented behavior (no TODOs or out-of-date stubs). The documentation and code are in sync.

* \[ \] **No Regressions:** Core regression suite passes – the feature didn’t break existing functionality (all previous tests still pass).

* \[ \] **Coding Standards Met:** The implementation follows the project conventions (e.g. no forbidden coordination patterns[\[2\]](file://file-AgNPwiG9jBq57Aahvq66nU#:~:text=), respects architecture constraints, proper error handling, etc.).

Before declaring the feature “done,” the team can go through this checklist. It is largely an extension of our current practice (much of this is implicitly done, like running tests), but writing it down ensures a *consistent, minimal-effort verification*. Many of these items can be automated: for instance, CI ensures tests pass and no regressions, while a simple script could check that each requirement ID is referenced in at least one test file (verifying traceability). The checklist thus complements automation by covering both automated and manual verification steps in one place.

## Automating Spec Compliance and Traceability

Finally, to truly minimize manual effort, we recommend a couple of lightweight automation ideas that align with our documentation-driven development:

* **Spec vs. Code Consistency Check:** Develop a small tool or script to parse the design.md for code blocks and verify they are implemented in the codebase. For example, if design.md contains a trait method fn update\_message(...) \-\> Result\<…\>[\[14\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20Updates%20an%20existing%20message,Updated%20message)[\[15\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20,Result%3CMessage%3CPersisted%3E%2C%20MessageError), the script can check that the codebase has a corresponding function signature. This could be as simple as grepping the repository for the function names or using Rust’s reflection (if available for tests). This ensures that if an interface stub in the doc is changed, the code is updated (and vice versa). In practice, running this script as part of CI or a pre-merge check would prompt developers to keep docs and code in sync.

* **Requirement Coverage Mapping:** As mentioned, use the requirement IDs in test code. We could adopt a convention in test functions or comments, e.g., \#\[test\] fn test\_emoji\_enlarge\_req1\_9() { … } or inside the test, a comment // Covers: Req 1.9. A simple grep or a more structured tool can scan all tests and build a list of covered requirement IDs, then compare to the list in requirements.md. This can be automated in CI. If any criterion isn’t referenced by a test, we get a warning. This automation directly enforces **full behavioral coverage** of user journeys – every user story acceptance criterion must have at least one test exercising it.

* **Template-driven Code Generation:** We can incorporate the aforementioned Markdown templates into a tool that uses our LLM of choice (perhaps via a prompt or fine-tuned model) to stub out code. For instance, a CLI tool could take a feature spec MD and output initial Rust files (with unimplemented\! functions and skeleton tests). This reduces the boilerplate the developer writes. Because the templates ensure a consistent format, this process can be repeated reliably for each new feature. Over time, as our library of specs grows, the LLM is “trained” on our style and produces increasingly on-convention code – maximizing reuse of our established patterns.

* **One-Command Test Runner:** Wrap the full-flow integration tests in a convenient script or Cargo command. For example, define custom Cargo aliases or a Makefile target for each major user journey (or use tags). The PM or dev can run make test-feature-x without remembering the exact test name. This is a trivial automation, but it lowers the barrier for non-developers to run tests, and encourages frequent full-flow testing. We will document these commands in the README or architecture.md for visibility.

By implementing these automation ideas, we ensure our **specs remain the single source of truth**. The development process becomes: write spec \-\> generate/check code \-\> run tests \-\> update spec if needed \-\> done. The feedback loops (spec-to-code and tests-to-requirements) are tightened with tooling, reducing human error and effort.

---

**Conclusion:** These documentation updates turn our design, requirements, tasks, and architecture docs into a cohesive system for TDD and feature specification. Each MD file will not only specify what the system *should* do, but also exactly *how we will verify it does it*. This aligns perfectly with our type-driven TDD philosophy (interfaces and contracts first) and the pragmatic Rails-parity approach. By providing structured templates and examples, we make it easy to spin up new features with minimal guesswork – the format itself guides the implementation. Moreover, the docs double as templates for LLMs, meaning we can leverage AI to generate a lot of the repetitive code and test scaffolding consistent with our conventions. The end result is a highly maintainable workflow where writing a new feature spec automatically sets up the development and QA process, ensuring completeness. It’s a small upfront investment in documentation structure that will pay off with faster development (less manual coding) and more reliable outcomes (every user journey fully tested).

With these changes, whenever a PM asks “Does Feature X fully work?”, the developer can confidently run the one-command test and reply, “Yes – here’s the green test proof,” knowing that our docs and tests have systematically covered all aspects of the feature. This marries the documentation and implementation in a deeply integrated, efficient way.

---

[\[1\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=This%20document%20defines%20the%20complete,first%20success%20and%20architectural%20correctness) [\[3\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=MessageService%20) [\[4\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20,Updates%20FTS5%20search%20index) [\[5\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2A%2ANext%20Steps%3A%2A%2A%201.%20Create%20property,guided%20implementation%20following%20these%20contracts) [\[6\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=,Arguments) [\[7\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20%2A%20%60client_message_id%60%20,Side%20Effects) [\[12\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=) [\[13\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20,to%20room%20subscribers%20via%20WebSocket) [\[14\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20Updates%20an%20existing%20message,Updated%20message) [\[15\]](file://file-859DZ6KQVD5KTnp3SH7eZe#:~:text=%2F%2F%2F%20,Result%3CMessage%3CPersisted%3E%2C%20MessageError) design.md

[file://file-859DZ6KQVD5KTnp3SH7eZe](file://file-859DZ6KQVD5KTnp3SH7eZe)

[\[2\]](file://file-AgNPwiG9jBq57Aahvq66nU#:~:text=) requirements.md

[file://file-AgNPwiG9jBq57Aahvq66nU](file://file-AgNPwiG9jBq57Aahvq66nU)

[\[8\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=0) [\[9\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=Success%20Metrics) [\[10\]](file://file-MMwW3y7rwpmnwBZXLG9nqK#:~:text=Performance%20Requirements%20%28Rails,100MB) tasks.md

[file://file-MMwW3y7rwpmnwBZXLG9nqK](file://file-MMwW3y7rwpmnwBZXLG9nqK)

[\[11\]](file://file-PW3T15a4YVXCsZ2Xe8esJ8#:~:text=,Verify%20behavior%20matches%20Rails%20ActionCable) architecture.md

[file://file-PW3T15a4YVXCsZ2Xe8esJ8](file://file-PW3T15a4YVXCsZ2Xe8esJ8)