flowchart TD
    Display["🎯 Display<br/>(Trait)<br/><i>src/lib.rs</i>"]
    User["📦 User<br/>(Struct)<br/><i>src/lib.rs</i>"]
    main["🔧 main<br/>(Function)<br/><i>src/main.rs</i>"]

    User -.-> Display
    main --> User
    main --> create_user
    main --> process_data
