package main

import "core:math"
import rl "vendor:raylib"

W :: 640
H :: 480
PI :: 3.14159265358979

main :: proc() {
	rl.InitWindow(W, H, "sphere view")
	rl.SetTargetFPS(60)

	dir := [3]f32{1, 2, 3}
	dir = rl.Vector3Normalize(dir)

	cam: rl.Camera
	cam.position.z = 10
	cam.fovy = 45
	cam.up.y = 1

	for !rl.WindowShouldClose() {
		t := f32(rl.GetTime())
		r: f32 = 1
		d := 3 * r + math.sin_f32(t)
		td := math.sqrt_f32(d * d - r * r)
		ty := r * td / d
		tx := math.sqrt_f32(r * r - ty * ty)
		m := ty / (tx - d)
		b := -m * d
		c := math.atanh(m) / PI * 180
		m2 := -1.3 * b / d


		cam.position = dir * d
		cam.fovy = 2 * c

		rl.BeginDrawing()
		defer rl.EndDrawing()

		rl.ClearBackground(rl.RAYWHITE)

		rl.BeginMode3D(cam)

		rl.DrawSphereWires(0, r, 10, 10, rl.GREEN)

		rl.EndMode3D()

		rl.DrawFPS(10, 10)
	}
}

