bevy build --release --yes web --bundle

cd target/bevy_web/web-release
zip -r bevy_game.zip bevy_game
mv bevy_game.zip ../../../
cd ../../../
