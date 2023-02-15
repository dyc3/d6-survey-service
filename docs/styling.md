# Styling Document

This document contains general rules to follow when styling components/pages throughout the project.


## Fonts

The general font to be used is **DM SANS**. You can find variations of the font at the link below.
> https://fonts.google.com/specimen/DM+Sans

| Descriptions and Subtext | Navigation Elements | Questions, Text Boxes, Buttons| Title Text |
|--|--|--|--|
| Normal 400 | Medium 500 | Medium 500|Bold 700 |

## Colors

The color schema for the entire site should be consistent to ensure an encapsulating experience. We also want to try to keep it a bit lively.

Our main color scheme revolves around:
-  [#426881  (66,104,129)](https://www.color-hex.com/color/426881)
![426881](https://color-hex.org/colors/426881.png)
- [#418053  (65,128,83)](https://www.color-hex.com/color/418053)
![418053](https://color-hex.org/colors/418053.png)

In order to maintain some sense of liveliness as we make components, borders and highlighted buttons should fill with a 50/50 gradient of these two colors. The CSS code for such a style is sampled below:

>background: linear-gradient(90deg, rgba(66,104,129,1) 0%, rgba(65,128,83,1) 100%);


In general:
- Use #426881 (the blue) for any title text, description text, etc.
- Use the provided gradient for any highlighted buttons, borders,  or long swathes of color such as a footer.


## Component Design
Components made should fit the following standards:
| Border Radius | Border Width |Color | Padding
|--|--|--|--|
| 10px | 3px | Use Above Gradient | Variable, but at least 9px at all times.