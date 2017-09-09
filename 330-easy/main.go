package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

type Point struct {
	X, Y float64
}

func (p Point) String() string {
	return fmt.Sprintf("(%.3f, %.3f)", p.X, p.Y)
}

type Circle struct {
	Center Point
	Radius float64
}

type BoundingBox struct {
	Left, Right, Bottom, Top float64
}

func (b *BoundingBox) Expand(left, right, bottom, top float64) {
	if left < b.Left {
		b.Left = left
	}
	if right > b.Right {
		b.Right = right
	}
	if bottom < b.Bottom {
		b.Bottom = bottom
	}
	if top > b.Top {
		b.Top = top
	}
}

func (b *BoundingBox) ToRectangle() *Rectangle {
	return &Rectangle{[4]Point{
		Point{b.Left, b.Bottom},
		Point{b.Left, b.Top},
		Point{b.Right, b.Top},
		Point{b.Right, b.Bottom},
	}}
}

type Rectangle struct {
	Corners [4]Point
}

func (r *Rectangle) String() string {
	var points []string
	for _, p := range r.Corners {
		points = append(points, p.String())
	}
	return strings.Join(points, ", ")
}

func main() {
	var circles []Circle
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		var c Circle
		fmt.Sscanf(scanner.Text(), "%f,%f,%f", &c.Center.X, &c.Center.Y, &c.Radius)
		circles = append(circles, c)
	}

	bb := BoundingBox{
		math.Inf(+1),
		math.Inf(-1),
		math.Inf(+1),
		math.Inf(-1),
	}
	for _, c := range circles {
		bb.Expand(
			c.Center.X-c.Radius,
			c.Center.X+c.Radius,
			c.Center.Y-c.Radius,
			c.Center.Y+c.Radius)
	}

	fmt.Println(bb.ToRectangle())
}
