use warp::Filter;
use rand::Rng;

static mut HIT_POINT: i32 = 30;

#[tokio::main]
async fn main() {

    // GET /
    let root = warp::path::end().map(|| unsafe{ root() });

    // GET /hi
    let hi = warp::path!("attack" / String).map(|name| unsafe{ attack(name) });

    let routes = warp::get().and(
        root
            .or(hi)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 10000))
        .await;
}

unsafe fn root() -> String {
    return format!("HitPoint: {}", HIT_POINT);
}

unsafe fn attack(name: String) -> String {
    let critical = rand::thread_rng().gen_range(0..3);
    let power = if critical == 2 {
        rand::thread_rng().gen_range(15..20)
    } else {
        rand::thread_rng().gen_range(4..8)
    };
    HIT_POINT -= power;
    let message = if HIT_POINT < 0 {
        format!("{}のこうげき！\nなにかよくわからないやつに {} のダメージ！\nよくわからないけど たおした！", name, power)
    } else {
        let critical_message = if critical == 2 {
            "かいしんのいちげき！！！\n"
        } else {
            ""
        };
        format!("{}のこうげき！\n{}なにかよくわからないやつに {} のダメージ！", name, critical_message, power)
    };
    return message;
}
