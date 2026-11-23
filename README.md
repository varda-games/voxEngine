# Vox(el) Engine
![version](https://img.shields.io/crates/v/vox_engine?style=for-the-badge)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/varda-games/voxEngine/static.yml?style=for-the-badge&label=docs)
![Static Badge](https://img.shields.io/badge/made%20with-rust-orange?style=for-the-badge&logo=rust)
![GitHub License](https://img.shields.io/github/license/varda-games/voxEngine?style=for-the-badge)



# why a custom engine?

tl;dr: over-ambitious and kinda dumb + otc game engines cant do what i want for same effort.

well i wanted to create a voxel based rogue-lite game, and i wanted to use  
proper voxel rendering techniques. initially i used traditional rendering techniques to achieve this,  
but since implementing vulkan, i have moved to using a raymarcher to render my voxels. initially my  
reasoning behind starting this project was that i wanted to learn the ins and outs of "over-the-counter"  
game engines like unity or unreal, but at this stage it might genuinely be more difficult to implement  
the rendering style i want in a game engine like unity.

# Want to use this for yourself?

add:

``` toml
  voxEngine = "0.5.1"
  vulkano = "0.35.2"
```

to your `Cargo.toml`

**also** check out the [docs](https://varda-games.github.io/voxEngine/vox_engine/index.html)

## dev logs

join the varda-games [discord](https://discord.gg/7X3Bvun4HU)

check out [voxEngine devlogs](https://youtube.com/playlist?list=PLkGaFuDyjwONmwLIEOyiIF43p4a-F6Qlz&si=TiN8CTq6OB9KWOlf)

i also do devstreams occasionally on [twitch](https://twitch.tv/veersinghlive)
