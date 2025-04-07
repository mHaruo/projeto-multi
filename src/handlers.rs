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
        given_today: 0,
        last_given_today: None,
        badge: "nenhum".to_string()
    };
    add_user(user);
    HttpResponse::Created().finish()
}


#[get("/users")]async fn list_users() -> impl Responder {
    HttpResponse::Ok().json(get_users())
}


#[post("/users/{from_id}/star/{to_id}")]
async fn star_user(path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let (from_id, to_id) = path.into_inner();

    match give_star(from_id, to_id) {
        Some(cost) => HttpResponse::Ok().body(format!("Star given! Cost: {:.2}", cost)),
        None => HttpResponse::NotFound().body("User not found or daily limit reached."),
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
                "stars": u.stars,
                "badge": u.badge
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
