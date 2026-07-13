use super::*;

pub fn builder() -> Box<dyn State> {
    Box::new(Draft {})
}

pub trait State {
    fn name(&self) -> &str;

    fn id(&self) -> i64;
    
    fn can_review(&self) -> Result<bool, PostError> {
        Err(PostError::ReviewRequest(
            format!("in state of {}, you can not request for a review!", self.name())))
    }

    fn can_modify(&self) -> Result<bool, PostError> {
        Err(PostError::ModifyRequest(
            format!("in state of {}, you can not modify the post!", self.name())))
    }

    fn backward_states(&self) -> Vec<Box<dyn State>> {
        vec![]
    }
    
    fn forward_states(&self) -> Vec<Box<dyn State>> {
        vec![]
    }
}

struct Draft;

struct PendingReview;

struct Approved;

struct Published;


impl State for Draft {
    fn name(&self) -> &str {
        "draft"
    }

    fn id(&self) -> i64 {
        1i64
    }
    
    fn can_review(&self) -> Result<bool, PostError> {
        Ok(true)
    }

    fn can_modify(&self) -> Result<bool, PostError> {
        Ok(true)
    }
    
    fn forward_states(&self) -> Vec<Box<dyn State>> {
        vec![Box::new(PendingReview {})]
    }    
}

impl State for PendingReview {
    fn name(&self) -> &str {
        "pending review"
    }

    fn id(&self) -> i64 {
        2i64
    }
    
    fn can_modify(&self) -> Result<bool, PostError> {
        Ok(true)
    }

    fn backward_states(&self) -> Vec<Box<dyn State>> {
        vec![Box::new(Draft {})]
    }
    
    fn forward_states(&self) -> Vec<Box<dyn State>> {
        vec![Box::new(Approved {})]
    }

}

impl State for Approved {
    fn name(&self) -> &str {
        "approved"
    }
    
    fn id(&self) -> i64 {
        3i64
    }
    
    fn backward_states(&self) -> Vec<Box<dyn State>> {
        vec![]
    }
    
    fn forward_states(&self) -> Vec<Box<dyn State>> {
        vec![Box::new(Published {})]
    }

}

impl State for Published {
    fn name(&self) -> &str {
        "published"
    }

    fn id(&self) -> i64 {
        4i64
    }
}
