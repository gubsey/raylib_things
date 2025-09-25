package main

import "core:fmt"
import "core:math"
import "core:strings"
import rl "vendor:raylib"

W :: 1000
H :: W
PI :: 3.14159265358979

main :: proc() {
	rl.InitWindow(W, H, "sphere view")
	rl.SetTargetFPS(60)

	dir := [3]f32{3, 5, 4}
	dir = rl.Vector3Normalize(dir)

	cam: rl.Camera
	cam.up.x = 1
	cam.up.z = 1

	for !rl.WindowShouldClose() {
		t := f32(rl.GetTime()) / 2
		zh: f32 = 5
		zh2 := zh * 2
		r: f32 = 1
		// d: f32 = math.sqrt_f32(2 * r * r) + t
		// d: f32 = r + (math.mod(t, zh2) if math.mod(t, zh2) <= zh else -math.mod(t, zh2) + zh2)
		d: f32 = r + math.abs(math.tan(t))
		// d: f32 = 1.1

		td := math.sqrt((d * d) - (r * r))
		ty := (r * td) / d
		tx := math.sqrt((r * r) - (ty * ty))
		m := ty / (tx - d)
		b := -m * d
		c := -math.atan(m * 1.1) * 360 / PI
		fmt.printf(
			"r: %f\nd: %f\ntd: %f\nty: %f\ntx: %f\nm: %f\nb: %f\nc: %f\n\n",
			r,
			d,
			td,
			ty,
			tx,
			m,
			b,
			c,
		)


		cam.position = dir * d
		cam.fovy = c

		rl.BeginDrawing()
		defer rl.EndDrawing()

		rl.ClearBackground(rl.BROWN)

		rl.BeginMode3D(cam)

		rl.DrawSphereWires(0, r, 30, 30, rl.GREEN)
		//rl.DrawPlane([3]f32{0, tx * 0.9, 0}, [2]f32{2 * r, 2 * r}, rl.BROWN)
		rl.EndMode3D()

		builder: strings.Builder
		defer strings.builder_destroy(&builder)
		wb := strings.to_writer(&builder)
		fmt.wprintf(wb, "d: %.2f\nfov: %.2f", d, cam.fovy)
		cstr, _ := strings.to_cstring(&builder)
		rl.DrawText(cstr, 10, 10, W / 20, rl.GREEN)
	}
}

