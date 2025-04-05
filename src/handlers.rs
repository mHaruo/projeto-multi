use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::models::{NewUser, User};
use crate::storage::{add_user, get_users, give_star};

#[post("/users")]
async fn create_user(new_user: web::Json<NewUser>) -> impl Responder {
    let user = User {
        id: Uuid::new_v4(),
        name: new_user.name.clone(),
        linkedin: new_user.linkedin.clone(),
        github: new_user.github.clone(),
        twitter: new_user.twitter.clone(),
        stars: 0,
    };
    add_user(user);
    HttpResponse::Created().finish()
}

#[get("/users")]async fn list_users() -> impl Responder {
    HttpResponse::Ok().json(get_users())
}

#[post("/users/{id}/star")]
async fn star_user(id: web::Path<Uuid>) -> impl Responder {
    if give_star(id.into_inner()) {
        HttpResponse::Ok().body("Star given!")
    } else {
        HttpResponse::NotFound().body("User not found.")
    }
}

#[get("/ranking")]
async fn ranking() -> impl Responder {
    let mut users = get_users();
    users.sort_by(|a, b| b.stars.cmp(&a.stars)); // Ordena do maior para o menor

    let ranking: Vec<_> = users
        .into_iter()
        .map(|u| {
            serde_json::json!({
                "name": u.name,
                "stars": u.stars
            })
        })
        .collect();

    HttpResponse::Ok().json(ranking)
}



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    cfg.service(list_users);
    cfg.service(star_user);
    cfg.service(ranking);

}
