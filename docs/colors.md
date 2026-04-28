# Colors

You can give each snake a custom color in `config.yaml` using the `color` field.
```yaml
players:
    - My Snake:
        type: custom
        color: pacific blue
        cmd: python
        args:
          - my_snake.py
```

## Accepted formats

The `color` field accepts either a hex string or a Crayola color name.

### Hex strings

Standard 6-digit hex color codes (with or without a leading `#`)
```yaml
color: "#fcd667"
color: fcd667
```

3-digit shorthand (e.g. `#fd6`) also works

### Crayola names

[Names of Crayola crayons](https://en.wikipedia.org/wiki/List_of_Crayola_crayon_colors) can also be used.
<details>
<summary>Complete color list</summary>

| Color | Hex |
|---|---|
| ![](https://placehold.co/15x15/ED0A3F/ED0A3F.png) Red | #ED0A3F |
| ![](https://placehold.co/15x15/C32148/C32148.png) Maroon | #C32148 |
| ![](https://placehold.co/15x15/FD0E35/FD0E35.png) Scarlet | #FD0E35 |
| ![](https://placehold.co/15x15/C62D42/C62D42.png) Brick Red | #C62D42 |
| ![](https://placehold.co/15x15/CC474B/CC474B.png) English Vermilion | #CC474B |
| ![](https://placehold.co/15x15/CC3336/CC3336.png) Madder Lake | #CC3336 |
| ![](https://placehold.co/15x15/E12C2C/E12C2C.png) Permanent Geranium Lake | #E12C2C |
| ![](https://placehold.co/15x15/D92121/D92121.png) Maximum Red | #D92121 |
| ![](https://placehold.co/15x15/B94E48/B94E48.png) Indian Red | #B94E48 |
| ![](https://placehold.co/15x15/FF5349/FF5349.png) Orange-Red | #FF5349 |
| ![](https://placehold.co/15x15/FE4C40/FE4C40.png) Sunset Orange | #FE4C40 |
| ![](https://placehold.co/15x15/FE6F5E/FE6F5E.png) Bittersweet | #FE6F5E |
| ![](https://placehold.co/15x15/B33B24/B33B24.png) Dark Venetian Red | #B33B24 |
| ![](https://placehold.co/15x15/CC553D/CC553D.png) Venetian Red | #CC553D |
| ![](https://placehold.co/15x15/E6735C/E6735C.png) Light Venetian Red | #E6735C |
| ![](https://placehold.co/15x15/FF9980/FF9980.png) Vivid Tangerine | #FF9980 |
| ![](https://placehold.co/15x15/E58E73/E58E73.png) Middle Red | #E58E73 |
| ![](https://placehold.co/15x15/FF7034/FF7034.png) Burnt Orange | #FF7034 |
| ![](https://placehold.co/15x15/FF681F/FF681F.png) Red-Orange | #FF681F |
| ![](https://placehold.co/15x15/FF8833/FF8833.png) Orange | #FF8833 |
| ![](https://placehold.co/15x15/FFB97B/FFB97B.png) Macaroni and Cheese | #FFB97B |
| ![](https://placehold.co/15x15/ECAC76/ECAC76.png) Middle Yellow Red | #ECAC76 |
| ![](https://placehold.co/15x15/E77200/E77200.png) Mango Tango | #E77200 |
| ![](https://placehold.co/15x15/FFAE42/FFAE42.png) Yellow-Orange | #FFAE42 |
| ![](https://placehold.co/15x15/F2BA49/F2BA49.png) Maximum Yellow Red | #F2BA49 |
| ![](https://placehold.co/15x15/FBE7B2/FBE7B2.png) Banana Mania | #FBE7B2 |
| ![](https://placehold.co/15x15/F2C649/F2C649.png) Maize | #F2C649 |
| ![](https://placehold.co/15x15/F8D568/F8D568.png) Orange-Yellow | #F8D568 |
| ![](https://placehold.co/15x15/FCD667/FCD667.png) Goldenrod | #FCD667 |
| ![](https://placehold.co/15x15/FED85D/FED85D.png) Dandelion | #FED85D |
| ![](https://placehold.co/15x15/FBE870/FBE870.png) Yellow | #FBE870 |
| ![](https://placehold.co/15x15/F1E788/F1E788.png) Green-Yellow | #F1E788 |
| ![](https://placehold.co/15x15/FFEB00/FFEB00.png) Middle Yellow | #FFEB00 |
| ![](https://placehold.co/15x15/B5B35C/B5B35C.png) Olive Green | #B5B35C |
| ![](https://placehold.co/15x15/ECEBBD/ECEBBD.png) Spring Green | #ECEBBD |
| ![](https://placehold.co/15x15/FAFA37/FAFA37.png) Maximum Yellow | #FAFA37 |
| ![](https://placehold.co/15x15/FFFF99/FFFF99.png) Canary | #FFFF99 |
| ![](https://placehold.co/15x15/FFFF9F/FFFF9F.png) Lemon Yellow | #FFFF9F |
| ![](https://placehold.co/15x15/D9E650/D9E650.png) Maximum Green Yellow | #D9E650 |
| ![](https://placehold.co/15x15/ACBF60/ACBF60.png) Middle Green Yellow | #ACBF60 |
| ![](https://placehold.co/15x15/AFE313/AFE313.png) Inchworm | #AFE313 |
| ![](https://placehold.co/15x15/BEE64B/BEE64B.png) Light Chrome Green | #BEE64B |
| ![](https://placehold.co/15x15/C5E17A/C5E17A.png) Yellow-Green | #C5E17A |
| ![](https://placehold.co/15x15/5E8C31/5E8C31.png) Maximum Green | #5E8C31 |
| ![](https://placehold.co/15x15/7BA05B/7BA05B.png) Asparagus | #7BA05B |
| ![](https://placehold.co/15x15/9DE093/9DE093.png) Granny Smith Apple | #9DE093 |
| ![](https://placehold.co/15x15/63B76C/63B76C.png) Fern | #63B76C |
| ![](https://placehold.co/15x15/4D8C57/4D8C57.png) Middle Green | #4D8C57 |
| ![](https://placehold.co/15x15/3AA655/3AA655.png) Green | #3AA655 |
| ![](https://placehold.co/15x15/6CA67C/6CA67C.png) Medium Chrome Green | #6CA67C |
| ![](https://placehold.co/15x15/5FA777/5FA777.png) Forest Green | #5FA777 |
| ![](https://placehold.co/15x15/93DFB8/93DFB8.png) Sea Green | #93DFB8 |
| ![](https://placehold.co/15x15/33CC99/33CC99.png) Shamrock | #33CC99 |
| ![](https://placehold.co/15x15/1AB385/1AB385.png) Mountain Meadow | #1AB385 |
| ![](https://placehold.co/15x15/29AB87/29AB87.png) Jungle Green | #29AB87 |
| ![](https://placehold.co/15x15/00CC99/00CC99.png) Caribbean Green | #00CC99 |
| ![](https://placehold.co/15x15/00755E/00755E.png) Tropical Rain Forest | #00755E |
| ![](https://placehold.co/15x15/8DD9CC/8DD9CC.png) Middle Blue Green | #8DD9CC |
| ![](https://placehold.co/15x15/01786F/01786F.png) Pine Green | #01786F |
| ![](https://placehold.co/15x15/30BFBF/30BFBF.png) Maximum Blue Green | #30BFBF |
| ![](https://placehold.co/15x15/00CCCC/00CCCC.png) Robin's Egg Blue | #00CCCC |
| ![](https://placehold.co/15x15/008080/008080.png) Teal Blue | #008080 |
| ![](https://placehold.co/15x15/8FD8D8/8FD8D8.png) Light Blue | #8FD8D8 |
| ![](https://placehold.co/15x15/95E0E8/95E0E8.png) Aquamarine | #95E0E8 |
| ![](https://placehold.co/15x15/6CDAE7/6CDAE7.png) Turquoise Blue | #6CDAE7 |
| ![](https://placehold.co/15x15/2D383A/2D383A.png) Outer Space | #2D383A |
| ![](https://placehold.co/15x15/76D7EA/76D7EA.png) Sky Blue | #76D7EA |
| ![](https://placehold.co/15x15/7ED4E6/7ED4E6.png) Middle Blue | #7ED4E6 |
| ![](https://placehold.co/15x15/0095B7/0095B7.png) Blue-Green | #0095B7 |
| ![](https://placehold.co/15x15/009DC4/009DC4.png) Pacific Blue | #009DC4 |
| ![](https://placehold.co/15x15/02A4D3/02A4D3.png) Cerulean | #02A4D3 |
| ![](https://placehold.co/15x15/47ABCC/47ABCC.png) Maximum Blue | #47ABCC |
| ![](https://placehold.co/15x15/2EB4E6/2EB4E6.png) Blue (I) | #2EB4E6 |
| ![](https://placehold.co/15x15/339ACC/339ACC.png) Cerulean Blue | #339ACC |
| ![](https://placehold.co/15x15/93CCEA/93CCEA.png) Cornflower | #93CCEA |
| ![](https://placehold.co/15x15/2887C8/2887C8.png) Green-Blue | #2887C8 |
| ![](https://placehold.co/15x15/00468C/00468C.png) Midnight Blue | #00468C |
| ![](https://placehold.co/15x15/0066CC/0066CC.png) Navy Blue | #0066CC |
| ![](https://placehold.co/15x15/1560BD/1560BD.png) Denim | #1560BD |
| ![](https://placehold.co/15x15/0066FF/0066FF.png) Blue (III) | #0066FF |
| ![](https://placehold.co/15x15/A9B2C3/A9B2C3.png) Cadet Blue | #A9B2C3 |
| ![](https://placehold.co/15x15/C3CDE6/C3CDE6.png) Periwinkle | #C3CDE6 |
| ![](https://placehold.co/15x15/4570E6/4570E6.png) Blue (II) | #4570E6 |
| ![](https://placehold.co/15x15/3C69E7/3C69E7.png) Bluetiful | #3C69E7 |
| ![](https://placehold.co/15x15/7A89B8/7A89B8.png) Wild Blue Yonder | #7A89B8 |
| ![](https://placehold.co/15x15/4F69C6/4F69C6.png) Indigo | #4F69C6 |
| ![](https://placehold.co/15x15/8D90A1/8D90A1.png) Manatee | #8D90A1 |
| ![](https://placehold.co/15x15/8C90C8/8C90C8.png) Cobalt Blue | #8C90C8 |
| ![](https://placehold.co/15x15/7070CC/7070CC.png) Celestial Blue | #7070CC |
| ![](https://placehold.co/15x15/9999CC/9999CC.png) Blue Bell | #9999CC |
| ![](https://placehold.co/15x15/ACACE6/ACACE6.png) Maximum Blue Purple | #ACACE6 |
| ![](https://placehold.co/15x15/766EC8/766EC8.png) Violet-Blue | #766EC8 |
| ![](https://placehold.co/15x15/6456B7/6456B7.png) Blue-Violet | #6456B7 |
| ![](https://placehold.co/15x15/3F26BF/3F26BF.png) Ultramarine Blue | #3F26BF |
| ![](https://placehold.co/15x15/8B72BE/8B72BE.png) Middle Blue Purple | #8B72BE |
| ![](https://placehold.co/15x15/652DC1/652DC1.png) Purple Heart | #652DC1 |
| ![](https://placehold.co/15x15/6B3FA0/6B3FA0.png) Royal Purple | #6B3FA0 |
| ![](https://placehold.co/15x15/8359A3/8359A3.png) Violet (II) | #8359A3 |
| ![](https://placehold.co/15x15/8F47B3/8F47B3.png) Medium Violet | #8F47B3 |
| ![](https://placehold.co/15x15/C9A0DC/C9A0DC.png) Wisteria | #C9A0DC |
| ![](https://placehold.co/15x15/BF8FCC/BF8FCC.png) Lavender (I) | #BF8FCC |
| ![](https://placehold.co/15x15/803790/803790.png) Vivid Violet | #803790 |
| ![](https://placehold.co/15x15/733380/733380.png) Maximum Purple | #733380 |
| ![](https://placehold.co/15x15/D6AEDD/D6AEDD.png) Purple Mountains' Majesty | #D6AEDD |
| ![](https://placehold.co/15x15/C154C1/C154C1.png) Fuchsia | #C154C1 |
| ![](https://placehold.co/15x15/FC74FD/FC74FD.png) Pink Flamingo | #FC74FD |
| ![](https://placehold.co/15x15/732E6C/732E6C.png) Violet (I) | #732E6C |
| ![](https://placehold.co/15x15/E667CE/E667CE.png) Brilliant Rose | #E667CE |
| ![](https://placehold.co/15x15/E29CD2/E29CD2.png) Orchid | #E29CD2 |
| ![](https://placehold.co/15x15/8E3179/8E3179.png) Plum | #8E3179 |
| ![](https://placehold.co/15x15/D96CBE/D96CBE.png) Medium Rose | #D96CBE |
| ![](https://placehold.co/15x15/EBB0D7/EBB0D7.png) Thistle | #EBB0D7 |
| ![](https://placehold.co/15x15/C8509B/C8509B.png) Mulberry | #C8509B |
| ![](https://placehold.co/15x15/BB3385/BB3385.png) Red-Violet | #BB3385 |
| ![](https://placehold.co/15x15/D982B5/D982B5.png) Middle Purple | #D982B5 |
| ![](https://placehold.co/15x15/A63A79/A63A79.png) Maximum Red Purple | #A63A79 |
| ![](https://placehold.co/15x15/A50B5E/A50B5E.png) Jazzberry Jam | #A50B5E |
| ![](https://placehold.co/15x15/614051/614051.png) Eggplant | #614051 |
| ![](https://placehold.co/15x15/F653A6/F653A6.png) Magenta | #F653A6 |
| ![](https://placehold.co/15x15/DA3287/DA3287.png) Cerise | #DA3287 |
| ![](https://placehold.co/15x15/FF3399/FF3399.png) Wild Strawberry | #FF3399 |
| ![](https://placehold.co/15x15/FBAED2/FBAED2.png) Lavender (II) | #FBAED2 |
| ![](https://placehold.co/15x15/FFB7D5/FFB7D5.png) Cotton Candy | #FFB7D5 |
| ![](https://placehold.co/15x15/FFA6C9/FFA6C9.png) Carnation Pink | #FFA6C9 |
| ![](https://placehold.co/15x15/F7468A/F7468A.png) Violet-Red | #F7468A |
| ![](https://placehold.co/15x15/E30B5C/E30B5C.png) Razzmatazz | #E30B5C |
| ![](https://placehold.co/15x15/FDD7E4/FDD7E4.png) Piggy Pink | #FDD7E4 |
| ![](https://placehold.co/15x15/E62E6B/E62E6B.png) Carmine | #E62E6B |
| ![](https://placehold.co/15x15/DB5079/DB5079.png) Blush | #DB5079 |
| ![](https://placehold.co/15x15/FC80A5/FC80A5.png) Tickle Me Pink | #FC80A5 |
| ![](https://placehold.co/15x15/F091A9/F091A9.png) Mauvelous | #F091A9 |
| ![](https://placehold.co/15x15/FF91A4/FF91A4.png) Salmon | #FF91A4 |
| ![](https://placehold.co/15x15/A55353/A55353.png) Middle Red Purple | #A55353 |
| ![](https://placehold.co/15x15/CA3435/CA3435.png) Mahogany | #CA3435 |
| ![](https://placehold.co/15x15/FEBAAD/FEBAAD.png) Melon | #FEBAAD |
| ![](https://placehold.co/15x15/F7A38E/F7A38E.png) Pink Sherbert | #F7A38E |
| ![](https://placehold.co/15x15/E97451/E97451.png) Burnt Sienna | #E97451 |
| ![](https://placehold.co/15x15/AF593E/AF593E.png) Brown | #AF593E |
| ![](https://placehold.co/15x15/9E5B40/9E5B40.png) Sepia | #9E5B40 |
| ![](https://placehold.co/15x15/87421F/87421F.png) Fuzzy Wuzzy | #87421F |
| ![](https://placehold.co/15x15/926F5B/926F5B.png) Beaver | #926F5B |
| ![](https://placehold.co/15x15/DEA681/DEA681.png) Tumbleweed | #DEA681 |
| ![](https://placehold.co/15x15/D27D46/D27D46.png) Raw Sienna | #D27D46 |
| ![](https://placehold.co/15x15/664228/664228.png) Van Dyke Brown | #664228 |
| ![](https://placehold.co/15x15/D99A6C/D99A6C.png) Tan | #D99A6C |
| ![](https://placehold.co/15x15/EDC9AF/EDC9AF.png) Desert Sand | #EDC9AF |
| ![](https://placehold.co/15x15/FFCBA4/FFCBA4.png) Peach | #FFCBA4 |
| ![](https://placehold.co/15x15/805533/805533.png) Burnt Umber | #805533 |
| ![](https://placehold.co/15x15/FDD5B1/FDD5B1.png) Apricot | #FDD5B1 |
| ![](https://placehold.co/15x15/EED9C4/EED9C4.png) Almond | #EED9C4 |
| ![](https://placehold.co/15x15/665233/665233.png) Raw Umber | #665233 |
| ![](https://placehold.co/15x15/837050/837050.png) Shadow | #837050 |
| ![](https://placehold.co/15x15/E6BC5C/E6BC5C.png) Raw Sienna (I) | #E6BC5C |
| ![](https://placehold.co/15x15/D9D6CF/D9D6CF.png) Timberwolf | #D9D6CF |
| ![](https://placehold.co/15x15/92926E/92926E.png) Gold (I) | #92926E |
| ![](https://placehold.co/15x15/E6BE8A/E6BE8A.png) Gold (II) | #E6BE8A |
| ![](https://placehold.co/15x15/C9C0BB/C9C0BB.png) Silver | #C9C0BB |
| ![](https://placehold.co/15x15/DA8A67/DA8A67.png) Copper | #DA8A67 |
| ![](https://placehold.co/15x15/C88A65/C88A65.png) Antique Brass | #C88A65 |
| ![](https://placehold.co/15x15/000000/000000.png) Black | #000000 |
| ![](https://placehold.co/15x15/736A62/736A62.png) Charcoal Gray | #736A62 |
| ![](https://placehold.co/15x15/8B8680/8B8680.png) Gray | #8B8680 |
| ![](https://placehold.co/15x15/C8C8CD/C8C8CD.png) Blue-Gray | #C8C8CD |
| ![](https://placehold.co/15x15/FFFFFF/FFFFFF.png) White | #FFFFFF |
| ![](https://placehold.co/15x15/FF355E/FF355E.png) Radical Red | #FF355E |
| ![](https://placehold.co/15x15/FD5B78/FD5B78.png) Wild Watermelon | #FD5B78 |
| ![](https://placehold.co/15x15/FF6037/FF6037.png) Outrageous Orange | #FF6037 |
| ![](https://placehold.co/15x15/FF9966/FF9966.png) Atomic Tangerine | #FF9966 |
| ![](https://placehold.co/15x15/FF9933/FF9933.png) Neon Carrot | #FF9933 |
| ![](https://placehold.co/15x15/FFCC33/FFCC33.png) Sunglow | #FFCC33 |
| ![](https://placehold.co/15x15/FFFF66/FFFF66.png) Laser Lemon | #FFFF66 |
| ![](https://placehold.co/15x15/FFFF66/FFFF66.png) Unmellow Yellow | #FFFF66 |
| ![](https://placehold.co/15x15/CCFF00/CCFF00.png) Electric Lime | #CCFF00 |
| ![](https://placehold.co/15x15/66FF66/66FF66.png) Screamin' Green | #66FF66 |
| ![](https://placehold.co/15x15/AAF0D1/AAF0D1.png) Magic Mint | #AAF0D1 |
| ![](https://placehold.co/15x15/50BFE6/50BFE6.png) Blizzard Blue | #50BFE6 |
| ![](https://placehold.co/15x15/FF6EFF/FF6EFF.png) Shocking Pink | #FF6EFF |
| ![](https://placehold.co/15x15/EE34D2/EE34D2.png) Razzle Dazzle Rose | #EE34D2 |
| ![](https://placehold.co/15x15/FF00CC/FF00CC.png) Hot Magenta | #FF00CC |
| ![](https://placehold.co/15x15/FF00CC/FF00CC.png) Purple Pizzazz | #FF00CC |
</details>

## Default colors

If you do not specify a `color`, snakes are assigned colors from the following list in order.
If you have more snakes than colors, the list cycles.


| Color | Hex |
|---|---|
| ![](https://placehold.co/15x15/C62D42/C62D42.png) Brick Red | #C62D42 |
| ![](https://placehold.co/15x15/5E8C31/5E8C31.png) Maximum Green | #5E8C31 |
| ![](https://placehold.co/15x15/02A4D3/02A4D3.png) Cerulean | #02A4D3 |
| ![](https://placehold.co/15x15/D27D46/D27D46.png) Raw Sienna | #D27D46 |
| ![](https://placehold.co/15x15/FFA6C9/FFA6C9.png) Carnation Pink | #FFA6C9 |
| ![](https://placehold.co/15x15/00CCCC/00CCCC.png) Robin's Egg Blue | #00CCCC |
| ![](https://placehold.co/15x15/FFAE42/FFAE42.png) Yellow-Orange | #FFAE42 |
| ![](https://placehold.co/15x15/C8509B/C8509B.png) Mulberry | #C8509B |
