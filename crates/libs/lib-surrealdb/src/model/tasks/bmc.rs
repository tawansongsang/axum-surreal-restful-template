use serde::de::DeserializeOwned;

use crate::{
    ctx::Ctx,
    model::{ModelManager, Result},
};

use super::{TasksForCreate, TasksForUpdate};

pub struct TasksBmc;

impl TasksBmc {
    pub async fn get<'de, E>(_ctx: &Ctx, mm: &ModelManager, id: &str) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        todo!()
    }

    pub async fn list<'de, E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        limit: Option<u32>,
        offset: Option<u32>,
        order: Option<bool>,
    ) -> Result<Vec<E>>
    where
        E: DeserializeOwned,
    {
        todo!()
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, task_id: &str) -> Result<()> {
        todo!()
    }

    pub async fn update<'de, E>(
        ctx: &Ctx,
        mm: &ModelManager,
        task_id: &str,
        task_for_update: TasksForUpdate,
    ) -> Result<E>
    where
        E: DeserializeOwned,
    {
        todo!()
    }

    pub async fn create<'de, E>(
        ctx: &Ctx,
        mm: &ModelManager,
        tasks_for_create: TasksForCreate,
    ) -> Result<E>
    where
        E: DeserializeOwned,
    {
        todo!()
    }
}
