package leetcode

// LeetCode #478 - Generate Random Point in a Circle
//
// Rejection sampling keeps a point from the bounding square when it falls inside the circle.
type CirclePoint struct {
	radius  float64
	xCenter float64
	yCenter float64
	rng     uint64
}

func NewCirclePoint(radius, xCenter, yCenter float64) *CirclePoint {
	return &CirclePoint{
		radius:  radius,
		xCenter: xCenter,
		yCenter: yCenter,
		rng:     0x123456789abcdef0,
	}
}

func (c *CirclePoint) RandPoint() []float64 {
	for {
		x := c.nextUnit()*2 - 1
		y := c.nextUnit()*2 - 1
		if x*x+y*y <= 1 {
			return []float64{c.xCenter + x*c.radius, c.yCenter + y*c.radius}
		}
	}
}

func (c *CirclePoint) nextUnit() float64 {
	c.rng ^= c.rng << 13
	c.rng ^= c.rng >> 7
	c.rng ^= c.rng << 17
	return float64(c.rng) / float64(^uint64(0))
}
