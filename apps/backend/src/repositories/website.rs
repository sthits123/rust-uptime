

pub fn get_website_by_id()->{
  let website=sqlx::query("SELECT * FROM website WHERE id= ? ");
  println!("The website is:{} ",website);
}
 

