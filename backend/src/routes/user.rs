use actix_web::{patch, post, web, HttpResponse, Responder };
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{Update, User};


#[post("/user")]
pub async fn insert(

    mut user: web::Json<User>,
    db: web::Data<PgPool>,

) ->impl Responder{

    println!("data recieved and will now be stored to database {:#?}", user);

    user.id = Some(Uuid::new_v4());
    user.joined_at = Some(Utc::now());
    
    //Querying to insert user to table.
    let result = sqlx::query(
        "INSERT INTO users (id, name, email, age, gender, joined_at, role)
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(user.id.unwrap())
    .bind(&user.name)
    .bind(&user.email)
    .bind(user.age as i32)
    .bind(user.gender.to_string())
    .bind(user.joined_at)
    .bind(user.role.to_string())
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(user.into_inner()),
        Err(e) => {
            eprintln!("DB insert error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to insert user")
        }
    }

}

#[patch("/user")]
pub async fn update(user : web::Json<Update> , db : web::Data<PgPool>) ->impl Responder {
    let result = sqlx::query("UPDATE users SET email = $1, name = $2 WHERE id = $3")
        .bind(&user.email)
        .bind(&user.name)
        .bind(user.id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(user.into_inner()),
        Err(e) => {
            eprintln!("DB updated error : {:#?}" ,e);
            HttpResponse::InternalServerError().body("Fialed to update the user data")
        }
    }
}
