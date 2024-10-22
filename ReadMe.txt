Objective as of 21 October 2024:
Create a rust program that can 
- read the zip file of an OSS folder whose absolute path is provided by the user (terminal input call folderA)
- place this extracted folder in a working-directory which is provided by the user (terminal input call folderB) OR process like a stream
- create a Rust a Rust aligned fast database (call it DatabaseA) in this folderB
- using this DatabaseA extract all the important information regarding code files in Java, JavaScript, Python, C++, C, Rust, Go
- using this DatabaseA extract all the paths, config files and other metadata
- using this DatabaseA encrytp all this information into a new txt file which is not human readable
- this file can be understood by LLMs and then can thus help us debug or understand the code the code that this OSS project represents
- this non-human-readable but LLM understandable txt file will be placed in the folderB
- this folderB will also contain a processProgress.txt file which will keep track of the progress of the program & have full deep statistics of the program which will help analyze the program progress for future improvements
- this folderB will also contain a log.txt file which will contain all significant observations of the program each time it is build or run, this will help us debug the program in the future at a cumulative level



Log:

- first version worked on 21 October 2024, I was ecstatic, it compressed the zip file processed it but could not capture all the metadata & depth of the code
- second version I tried to do ASTs & complicatded things and failed miserably so destroyed it as of 22 October 2024 morning 5 amuldotexe
- third version trying at 22 October 2024 morning 9 am

=========== update as of 22 Oct 2024 1300 hrs ======
The current implementation in main.rs has made significant progress towards the objectives:
1. ZIP file reading is implemented.
2. A Rust-aligned database (sled) is created and used.
3. Basic code analysis for multiple languages is implemented.
4. Progress tracking and logging are partially implemented.

However, several key features are still missing or incomplete:
1. Extraction to a working directory is not implemented (currently processing as a stream).
2. Metadata extraction is limited and needs expansion.
3. The output is not encrypted or formatted for LLM understanding.
4. The progress and log files don't exactly match the specifications.
5. AST parsing is very basic and limited.

The implementation provides a solid foundation but requires additional work to fully meet the objectives outlined in the README. The next steps should focus on implementing the missing features and refining the existing ones to align more closely with the project requirements.
