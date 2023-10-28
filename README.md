# Foodfight Tactics

Foodfight Tactics has long been a meme within the Teamfight Tactics community as a set that would happen if Kent became the lead designer of TFT. Since Mort is still the BDFL of TFT, Riot won't be making Foodfight Tactics.

Welcome the next best thing! Open source Foodfight Tactics! Build from the ground up with Rust to be **blazingly fast**, and better yet, completely web based! No more 2 week patch schedules and Riot Client issues haha!

On a serious note, if I violate some sort of policy and Riot sends me a cease and desist, I'm taking this down.

## Road Map

- [ ] Learn Rust
- [ ] Make web app
- [ ] Publish game
- [ ] Soju tries the game
- [ ] Get featured on TFT Clips
- [ ] Get infinite clout
- [ ] Anger Riot for trademark violation
- [ ] Take down website
- [ ] Apply to Riot Games as a SWE
- [ ] Get in because Foodfight Tactics was LIT
- [ ] Convince Mortdog to make Foodfight Tactics
- [ ] Everyone is happy, WOKEGE

## Dev Diary

This is my first project in Rust, and possibly my most ambitious project. Previous to this, my experience in Rust has been reading till chapter 13 of [The Book](https://rust-book.cs.brown.edu/ch13-02-iterators.html). This diary aims to serve the following purposes.

1. To document my progress in learning Rust, Rust Web Dev, WASM
2. To stay motivated and force myself to make a commit everyday
3. To look back and see how far I've come every once in a while to stroke my ego
4. To figure out which rabbit hole I dove in and wasted all my time on
5. Documentation is a good habit in general

### 2023-10-26

- Swapped from Yew to Leptos
  - [Yew vs Leptos](https://www.reddit.com/r/rust/comments/1526qo3/yew_or_leptos/)
  - Yew is not blazingly fast
  - Yew is basically react but in rust
- Setup [leptos](https://leptos-rs.github.io/leptos/) for frontend
- Setup [leptosfmt](https://github.com/bram209/leptosfmt) for my workspace
- Investigated why rust-analyzer is slow AF
  - https://github.com/rust-lang/rust-analyzer/issues/14258
  - Wasn't a neovim issue, was slow on VSC too
  - Guessing it's because leptos is huge so the startup analysis is slow

### 2023-10-27

- Learning HTML since I forgot about that part and dove head on into react [cheatsheet](https://web.stanford.edu/group/csp/cs21/htmlcheatsheet.pdf)
- Decided that I will implement functionality first before styling
- Added a login page
- Added a home page
- Initialized backend with axum
- Will be following this [video](https://www.youtube.com/watch?v=XZtlD_m59sM) until I figure out the authentication cookie setting part

### 2023-10-28

- Didn't get much done today, went out to explore earth instead.
- use `cargo watch -c -x run` to `clear` then `cargo run` every time a file changes
