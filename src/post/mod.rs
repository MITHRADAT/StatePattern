use std:: {rc::Rc,
           time::SystemTime,
           fmt::Display
};

mod state;

pub struct Post {
    author: String,
    header: String,
    body: String,
    state: Box<dyn state::State>,
    review_token: Option<Rc<Token>>
}

pub struct Brief<'a> {
    post: &'a Post
}

impl Post {
    pub fn new(author: &str, header: &str, body: &str) -> Self {
        Self {
            author: String::from(author),
            header: String::from(header),
            body: String::from(body),
            state: state::builder(),
            review_token: None
        }
    }

    pub fn peek(&self) -> Brief<'_> {
        Brief {
            post: self 
        }
    }

    pub fn request_for_review(&mut self) -> Result<(Rc<Token>, &String), PostError> {
        _ = self.state.can_review()?;
        let token = Rc::new(Token::new(SystemTime::now()));
        let token_clone = Rc::clone(&token);
        self.review_token = Some(token);
        Ok((token_clone, &self.body))
    }

    pub fn modify(&mut self, token: Rc<Token>,
                  header: Option<&str>, body: Option<&str>) -> Result<&Post, PostError>
    {
        if self.review_token.is_none() {
            return Err(PostError::InvalidToken(
                format!("you have to request for review first!")))
        }

        if body.is_none() && header.is_none() {
            return Err(PostError::InvalidFormat(
                format!("no modification sent to this post")));
        }
        
        let post_token = self.review_token.take().unwrap(); 
        if !Rc::ptr_eq(&post_token, &token) ||
            post_token.timestamp != token.timestamp
        {
            return Err(PostError::InvalidToken(
                format!("mismatch token! try to request for review!")))
        }
            
        self.state.can_modify()?;
            
        if let Some(new_header) = header {
            self.header = String::from(new_header);
            return Ok(self);
        }

        if let Some(new_body) = body {
            self.body = String::from(new_body);
            return Ok(self);
        }

        Err(PostError::Unknown)
    }

    pub fn state_manager(&mut self) -> StateManager<'_> {
        let mut targets = vec![];
        
        for state in self.state.backward_states() {
            targets.push(state)
        }
        
        for state in self.state.forward_states() {
            targets.push(state)
        }

        StateManager {
            post: self,
            targets: targets
        }
    }

    fn change_state(&mut self, target: Box<dyn state::State>) {
        self.state = target
    }
}


impl<'a> Brief<'a> {
    pub fn header(&self) -> &String {
        &self.post.header
    }
    
    pub fn author(&self) -> &String {
        &self.post.author
    }

    pub fn state(&self) -> &str {
        self.post.state.name()
    }
}

pub struct StateManager<'a> {
    post: &'a mut Post,
    targets: Vec<Box<dyn state::State>>
}

impl<'a> StateManager<'a> {
    pub fn transition(self, state_id: i64) -> Result<&'a Post, PostError> {
        let source_state_name = self.post.state.name();
        let target_state = self
            .targets
            .into_iter()
            .find(|state| state.id() == state_id);
        
        if let Some(new_state) = target_state {
            self.post.change_state(new_state);
            Ok(self.post)
        } else {
            Err(PostError::StateChange(
                format!("can not change state from {} to selected state!", source_state_name)))
        }
    }

    pub fn targets(&self) -> &Vec<Box<dyn state::State>> {
        &self.targets
    }
}

pub enum PostError {
    ReviewRequest(String),
    ModifyRequest(String),
    InvalidFormat(String),
    StateChange(String),
    InvalidToken(String),
    Unknown
}

impl Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostError::ReviewRequest(msg) |
            PostError::ModifyRequest(msg) |
            PostError::InvalidFormat(msg) |
            PostError::StateChange(msg) |
            PostError::InvalidToken(msg) => write!(f, "{}", msg),
            PostError::Unknown => write!(f, "an unknown error occured!")
        }
    }
}

pub struct Token {
    timestamp: SystemTime
}

impl Token {
    fn new(timestamp: SystemTime) -> Self {
        Self {
            timestamp
        }
    }
}
