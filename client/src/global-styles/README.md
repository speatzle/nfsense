# Global Styles
This folder contains stylesheets which contain rules that apply to everything in the project.

## Colors: Cascading Color Design System

CCDS uses OKLCH and a small set of *cascading semantic parameters* for elevation, interaction state and content emphasis to derive final lightness for every element's background, foreground, and border. They compose with *base values for lightness, hue and chroma* to create a perceptually uniform system where visual hierarchy emerges naturally from semantic markup. Theme switching becomes trivial by overriding only a small subset of base values.

The following diagram illustrates how colors are derived:
```mermaid
flowchart TD
  subgraph fcc[Per-Element Final Color]
    subgraph lc[Lightness Calculation]
      bgl["Background (--cl-bg-fin-l)"]
      bdl["Border (--cl-bd-fin-l)"]
      fgl["Foreground (--cl-fg-fin-l)"]
    end
    c["Chroma<br>(Tapered by Lightness)"]
    h[Hue]
    l[Lightness]
    fc["Final Colors (--cl-bg/fg/bd)"]
  end
  subgraph ep[Semantic Parameters]
    i["Interaction State (--cl-i)"]
    z["Surface Elevation (--cl-z)"]
    b["Base Color (--cl-base-*)"]
    e["Content Emphasis (--cl-e)"]
  end
  subgraph sp[Theme Parameters]
  subgraph bl[Base Lightnesses]
    bgbl["Background (--cl-bg-l)"]
    fgbl["Foreground (--cl-fg-l)"]
    bdbl["Border (--cl-bd-l)"]
  end
    subgraph bc[Base Colors]
    bp["Primary (--cl-primary)"]
    bn["Neutral (--cl-neutral)"]
    etc["Functional, Etc. (e.g. --cl-error, --cl-secondary, ...)"]
    end
    subgraph lo[Lightness Offsets]
      direction TD
      ioff["Interaction (--cl-ioff)"]
      zoff["Elevation (--cl-zoff)"]
      eoff["(De-)Emphasis (--cl-eoff)"]
    end
  end
  lc --> c
  lc --> l
  h --> fc
  c --> fc
  l --> fc
  b --> h
  b --> c
  
  ioff --X--- i
  zoff --X--- z
  eoff --X--- e
  
  bc --- b
  bl --> lc

  z --=--> bgl
  i --=--> bgl
  e --=--> bdl
  z --=--> bdl
  e --=--> fgl
```
