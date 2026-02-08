use futures::executor::block_on;

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main())

}

async fn async_main() {
    let sing = learn_and_sing();
    let dance = dance();
    futures::join!(sing, dance);
}

async fn learn_and_sing()  {
    let song = learn_song().await;
    sing_song(song).await;
}


struct Song;
async fn learn_song()->Song {
    Song{}
}
async fn sing_song(_song: Song){

}

async fn dance(){

}