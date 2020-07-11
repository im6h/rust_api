use crate::schema::links;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Link {
  pub id: i32,
  pub title: String,
  pub descripton: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "links"]
pub struct NewLink {
  pub title: String,
  pub descripton: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkList(pub Vec<Link>);

impl LinkList {
  pub fn list() -> Self {
    use crate::db_connection::establish_connection;
    use crate::schema::links::dsl::*;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;

    let connection = establish_connection();

    let result = links
      .limit(10)
      .load::<Link>(&connection)
      .expect("Error loading link");
    LinkList(result)
  }
}

impl NewLink {
  pub fn create(&self) -> Result<Link, diesel::result::Error> {
    use crate::db_connection::establish_connection;
    use diesel::RunQueryDsl;
    let connection = establish_connection();
    diesel::insert_into(links::table)
      .values(self)
      .get_result(&connection)
  }
}

impl Link {
  pub fn find(id: &i32) -> Result<Link, diesel::result::Error> {
    use crate::db_connection::establish_connection;
    use diesel::*;

    let connection = establish_connection();
    links::table.find(id).first(&connection)
  }
  pub fn delete(id: &i32) -> Result<(), diesel::result::Error> {
    use crate::db_connection::establish_connection;
    use crate::schema::links::dsl;
    use diesel::*;

    let connection = establish_connection();
    diesel::delete(dsl::links.find(id)).execute(&connection)?;
    Ok(())
  }
  pub fn update(id: &i32, new_link: &NewLink) -> Result<(), diesel::result::Error> {
    use crate::db_connection::establish_connection;
    use crate::schema::links::dsl;
    use diesel::*;

    let connection = establish_connection();
    diesel::update(dsl::links.find(id))
      .set(new_link)
      .execute(&connection)?;
    Ok(())
  }
}
