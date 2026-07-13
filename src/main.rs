use std::rc::Rc;

mod post;

fn main() {
    println!("state pattern is an object oriented design pattern");

    let mut post1 = post::Post::new("milad", "about rust", "rust is a strong programming language.\r\nIt even supports very useful features of object oriented languages.\r\n");

    let brief1 = post1.peek();
    println!("lets have a glance at post1:");
    println!("\tauthor:\r\n\t\t{}", brief1.author());
    println!("\tstate:\r\n\t\t{}", brief1.state());
    println!("\theader:\r\n\t\t{}\r\n", brief1.header());

    let review_result = post1.request_for_review();

    let mut token = None;
    let mut token_duplicate = None;
    let mut body = None;
    match review_result {
        Ok((t, b)) => {
            let t_duplicate = Rc::clone(&t);
            token = Some(t);
            token_duplicate = Some(t_duplicate);
            body = Some(b)
        },
        Err(err) => println!("{}", err)
    }

    println!("after successful review request now i can have body as follow:\r\nbody:\r\n\t{}"
             , body.unwrap());
    
    let new_body = format!("{} you just need to know what are you doing\r\n"
                           , body.unwrap());

    let modify_result1 = post1.modify(
        token.unwrap(), None, Some(new_body.as_str()));

    match modify_result1 {
        Ok(_) => println!("post modified successfully!"),
        Err(err) => println!("{}", err)
    }
    
    let modify_result2 = post1.modify(
        token_duplicate.unwrap(), Some("this wont work in rust"),
        Some("because post's token became empty after first modification"));

    match modify_result2 {
        Ok(_) => println!("post modified successfully by a forged token"),
        Err(err) => println!("forged token: {}", err)
    }


    let brief1 = post1.peek();
    println!("lets have a glance at post1 after modification:");
    println!("\tauthor:\r\n\t\t{}", brief1.author());
    println!("\tstate:\r\n\t\t{}", brief1.state());
    println!("\theader:\r\n\t\t{}\r\n", brief1.header());    

    let state_manager = post1.state_manager();
    let eligible_target_states = state_manager.targets();
    let selected_target_id = eligible_target_states[0].id();
    let result_transition = state_manager.transition(selected_target_id);

    match result_transition {
        Ok(post) => println!("post's state changed successfully. new state: {}",
                             post.peek().state()),
        Err(err) => println!("{}", err)
    }

    
    let state_manager = post1.state_manager();
    let eligible_target_states = state_manager.targets();
    let selected_target_id = eligible_target_states[1].id();
    let result_transition = state_manager.transition(selected_target_id);

    match result_transition {
        Ok(post) => println!("post's state changed successfully. new state: {}",
                             post.peek().state()),
        Err(err) => println!("{}", err)
    }
}
