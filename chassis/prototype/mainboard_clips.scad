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

difference() {
  cube([mount_depth_left, clearance + mainboard_thickness + clip_wall, clip_wall + groove_depth]);
  translate([0, clearance, clip_wall]) {
    cube([mount_depth_left, mainboard_thickness, groove_depth]);
  }
}

/*
difference() {
  cube([mount_depth_right, clearance + mainboard_thickness + clip_wall, clip_wall + groove_depth]);
  translate([0, clearance, clip_wall]) {
    cube([mount_depth_right, mainboard_thickness, groove_depth]);
  }
}
*/