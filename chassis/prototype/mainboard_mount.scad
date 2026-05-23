mainboard_thickness = 1.45;
mainboard_angle = 35;
mainboard_width = 75.15;
mainboard_depth = 53.7;
mount_depth_left = 7.45;
mount_depth_right = 30;
groove_depth = 5.65;
clearance = 2;
platform_wall = 4;
clip_wall = 1.6;

module triangle_bracket(base, height, wall) {
  hypotenuse = sqrt(height * height + base * base);
  scaled_height = height - wall * ((height + hypotenuse) / base);
  scaled_base = base - wall * ((base + hypotenuse) / height);
  linear_extrude(height = wall) {
    difference() {
      polygon([[0, 0], [base, 0], [0, height]]);
      polygon([
        [wall, wall],
        [scaled_base, wall],
        [wall, scaled_height]
      ]);
    }
  }
}

platform_height = mainboard_depth * sin(mainboard_angle);
platform_depth = mainboard_depth * cos(mainboard_angle);

triangle_bracket(platform_depth, platform_height, platform_wall);

translate([0, 0, mainboard_width - platform_wall]) {
  triangle_bracket(platform_depth, platform_height, platform_wall);
}

translate([0, 3 * (platform_height - platform_wall) / 5, 0]) {
  cube([platform_wall, platform_wall, mainboard_width]);
}

translate([0, (platform_height - platform_wall) / 5, 0]) {
  cube([platform_wall, platform_wall, mainboard_width]);
}

translate([3 * (platform_depth - platform_wall) / 5, 0, 0]) {
  cube([platform_wall, platform_wall, mainboard_width]);
}

translate([(platform_depth - platform_wall) / 5, 0, 0]) {
  cube([platform_wall, platform_wall, mainboard_width]);
}







