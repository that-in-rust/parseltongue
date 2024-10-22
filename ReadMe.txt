Objective as of 21 October 2024:
Create a rust program that can 
- read the zip file of an OSS folder whose absolute path is provided by the user (terminal input call folderA)
- place this extracted folder in a working-directory which is provided by the user (terminal input call folderB)
- create a Rust a Rust aligned fast database (call it DatabaseA) in this folderB
- using this DatabaseA extract all the important information regarding code files in Java, JavaScript, Python, C++, C, Rust, Go
- using this DatabaseA extract all the paths, config files and other metadata
- using this DatabaseA take inspiration from terdio to know what to read and what to ignore
- using this DatabaseA encrytp all this information into a new txt file which is not human readable
- this file can be understood by LLMs and then can thus help us debug or understand the code the code that this OSS project represents
- this non-human-readable but LLM understandable txt file will be placed in the folderB
- this folderB will also contain a processProgress.txt file which will keep track of the progress of the program & have full deep statistics of the program which will help analyze the program progress for future improvements
- this folderB will also contain a log.txt file which will contain all significant observations of the program each time it is build or run, this will help us debug the program in the future at a cumulative level



Log:

- first version worked on 21 October 2024, I was ecstatic, it compressed the zip file processed it but could not capture all the metadata & depth of the code
- second version I tried to do ASTs & complicatded things and failed miserably so destroyed it as of 22 October 2024 morning 5 amuldotexe
- third version trying at 22 October 2024 morning 9 am
