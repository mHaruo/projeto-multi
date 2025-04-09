use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::models::{Project, NewProject, Vote, VoteOption};
use crate::storage::get_users;

lazy_static! {
    pub static ref PROJECTS: Mutex<Vec<Project>> = Mutex::new(Vec::new());
}

#[post("/projects")]
pub async fn create_project(project: web::Json<NewProject>) -> impl Responder {
    let new_project = Project {
        id: Uuid::new_v4(),
        title: project.title.clone(),
        votes: Vec::new(),
    };

    PROJECTS.lock().unwrap().push(new_project);
    HttpResponse::Created().body("Project created")
}

#[get("/projects")]
pub async fn list_projects() -> impl Responder {
    let projects = PROJECTS.lock().unwrap();

    let summary: Vec<_> = projects.iter().map(|p| {
        let in_favor: u32 = p.votes.iter().filter(|v| matches!(v.in_favor, VoteOption::Sim)).map(|v| v.weight).sum();
        let against: u32 = p.votes.iter().filter(|v| matches!(v.in_favor, VoteOption::Nao)).map(|v| v.weight).sum();

        serde_json::json!({
            "id": p.id,
            "title": p.title,
            "votes_in_favor": in_favor,
            "votes_against": against,
            "total_votes": in_favor + against
        })
    }).collect();

    HttpResponse::Ok().json(summary)
}

#[derive(serde::Deserialize)]
pub struct VoteRequest {
    pub user_id: Uuid,
    pub vote: String,
}

#[post("/projects/vote/{project_id}")]
pub async fn vote(path: web::Path<Uuid>, vote_data: web::Json<VoteRequest>) -> impl Responder {
    let project_id = path.into_inner();
    let user_id = vote_data.user_id;
    let vote_option_str = vote_data.vote.to_lowercase();

    let users = get_users();
    let user = match users.iter().find(|u| u.id == user_id) {
        Some(u) => u,
        None => return HttpResponse::NotFound().body("Usuario nao encontrado"),
    };

    let in_favor = match vote_option_str.as_str() {
        "sim" => VoteOption::Sim,
        "nao" => VoteOption::Nao,
        _ => return HttpResponse::BadRequest().body("Voto deve ser 'Sim' ou 'Nao'"),
    };

    let weight = 1 + (user.stars / 5);

    let mut projects = PROJECTS.lock().unwrap();
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {

        if project.votes.iter().any(|v| v.user_id == user_id) {
            return HttpResponse::BadRequest().body("Usuario ja votou nesse projeto");
        }

        project.votes.push(Vote {
            user_id,
            weight,
            in_favor,
        });

        HttpResponse::Ok().body("Voto Registrado")
    } else {
        HttpResponse::NotFound().body("Projeto nao encontrado")
    }
}