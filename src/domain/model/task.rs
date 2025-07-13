use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error)]
pub enum TaskValidationError {
    #[error("Description cannot be empty")]
    EmptyDescription,
    #[error("Description cannot exceed {0} characters")]
    DescriptionTooLong(usize),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub completed: bool,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(value_type = String, format = DateTime)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Task {
    pub fn new(id: u64, description: String) -> Result<Self, TaskValidationError> {
        let now = chrono::Utc::now();
        let task = Self {
            id,
            description: description.clone(),
            completed: false,
            created_at: now,
            updated_at: now,
        };
        task.validate()?;
        Ok(task)
    }

    pub fn validate(&self) -> Result<(), TaskValidationError> {
        if self.description.trim().is_empty() {
            return Err(TaskValidationError::EmptyDescription);
        }
        
        if self.description.len() > 1000 {
            return Err(TaskValidationError::DescriptionTooLong(1000));
        }
        
        Ok(())
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.updated_at = chrono::Utc::now();
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
        self.updated_at = chrono::Utc::now();
    }

    pub fn update_description(&mut self, description: String) -> Result<(), TaskValidationError> {
        let original_description = self.description.clone();
        self.description = description;
        
        match self.validate() {
            Ok(()) => {
                self.updated_at = chrono::Utc::now();
                Ok(())
            }
            Err(e) => {
                self.description = original_description;
                Err(e)
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn is_pending(&self) -> bool {
        !self.completed
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct CreateTask {
    pub description: String,
}

impl CreateTask {
    pub fn new(description: String) -> Result<Self, TaskValidationError> {
        let create_task = Self { description };
        create_task.validate()?;
        Ok(create_task)
    }

    pub fn validate(&self) -> Result<(), TaskValidationError> {
        if self.description.trim().is_empty() {
            return Err(TaskValidationError::EmptyDescription);
        }
        
        if self.description.len() > 1000 {
            return Err(TaskValidationError::DescriptionTooLong(1000));
        }
        
        Ok(())
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct UpdateTask {
    pub description: Option<String>,
    pub completed: Option<bool>,
}

impl UpdateTask {
    pub fn new(description: Option<String>, completed: Option<bool>) -> Result<Self, TaskValidationError> {
        let update_task = Self { description, completed };
        update_task.validate()?;
        Ok(update_task)
    }

    pub fn validate(&self) -> Result<(), TaskValidationError> {
        if let Some(ref description) = self.description {
            if description.trim().is_empty() {
                return Err(TaskValidationError::EmptyDescription);
            }
            
            if description.len() > 1000 {
                return Err(TaskValidationError::DescriptionTooLong(1000));
            }
        }
        
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.description.is_none() && self.completed.is_none()
    }
}

 