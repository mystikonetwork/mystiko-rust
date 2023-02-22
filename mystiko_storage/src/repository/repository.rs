use crate::Model;

pub trait Repository {
  fn find_by_id(&self, id: String) -> Box<dyn Model>;
}