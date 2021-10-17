use warp::Filter;
use rand::Rng;
use std::env;

static mut HIT_POINT: i32 = 30;


#[tokio::main]
async fn main() {
    let port_key = "PORT";
    let default_port = 8088;
    let port = match env::var(port_key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!(
                    "the port number \"{}\" is invalid. default port will be used.",
                    val
                );
                default_port
            }
        },
        Err(_) => {
            println!(
                "\"{}\" is not defined in environment variables. default port will be used.",
                port_key
            );
            default_port
        }
    };

    let root = warp::path::end().map(|| unsafe{ root() });
    let kougeki = warp::path!("kougeki" / String).map(|name| unsafe{ kougeki(name) });
    let zaoriku = warp::path("zaoriku").map(|| unsafe{ zaoriku() });

    let routes = warp::get().and(
        root
            .or(kougeki)
            .or(zaoriku)
    );

    warp::serve(routes)
        .run(([0, 0, 0, 0], default_port))
        .await;
}

unsafe fn root() -> String {
    return format!("たたかう？ /kougeki/:name");
}

unsafe fn kougeki(name: String) -> String {
    if HIT_POINT < 1 {
        return format!("へんじがない。ただの　しかばね　のようだ\nふっかつのじゅもん\n/zaoriku");
    }
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

unsafe fn zaoriku() -> String {
    if HIT_POINT < 1 {
        HIT_POINT = 30;
        return format!("ザオリクを　となえた！\nなにかよくわからないやつは　いきかえった！");
    } else {
        return format!("ザオリクを　となえた！\nなにかよくわからないやつは　きかなかった！");
    }
}
