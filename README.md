# Thymine
A rust-powered GTK widget template language

Heavily inspired by [EWW](https://github.com/elkowar/eww) and [ags](https://github.com/Aylur/ags).

## Usage

```
thymine -h
```

## Example
```
{Window: $title "Hello World" 
    {Box: "v" $spacing 10 $class "Hello" 
        {Label: "Hello World" $wrap true $class "World"};
        {Button: "Click Me!!!" $onClick "notify-send hello" $class "World"};
        {Label: "Hello World" $wrap true $class "World"}
    }
}
```
