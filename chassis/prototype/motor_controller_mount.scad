use <pattern/pattern.scad>;
use <threads/threads.scad>;

$fn=100;

mainboard_width = 75.15;
stand_hole_radius = 1.5;
ctrl_board_width = 32;
ctrl_board_length = 34.95;
ctrl_board_hole_offsets = [
  [1.62 + stand_hole_radius, ctrl_board_length - stand_hole_radius - 1.38],
  [ctrl_board_width - stand_hole_radius - 1.28, ctrl_board_length - stand_hole_radius - 1.32],
  [1.45 + stand_hole_radius, 1.30 + stand_hole_radius],
  [ctrl_board_width - stand_hole_radius - 1.13, 1.40 + stand_hole_radius]
];
buck_length = 43.07;
buck_width = 21.34;
buck_hole_offsets = [
  [buck_width - stand_hole_radius - 0.7, buck_length - stand_hole_radius - 4.57],
  [1.11 + stand_hole_radius, 4.25 + stand_hole_radius]
];
wall = 1.6;

module post() {
  RodEnd(5, 8, thread_len=8, thread_diam=3);
  translate([0, 0, -wall]) {
    cylinder(h = wall, r = wall + stand_hole_radius + 1);
  }
}

module ctrl_board_posts() {
  for(i = [0:3]) {
    translate([ctrl_board_hole_offsets[i][0], ctrl_board_hole_offsets[i][1]]) {
      post();
      //circle(r = stand_hole_radius);
    }
  }
}

module ctrl_board() {  
  difference() {
    square([ctrl_board_width, ctrl_board_length]);
    ctrl_board_posts();
  }
}

module buck_posts() {
  for(i = [0:1]) {
    translate([buck_hole_offsets[i][0], buck_hole_offsets[i][1]]) {
      post();
      //circle(r = stand_hole_radius);
    }
  }
}

module buck() {
  difference() {
    square([buck_width, buck_length]);
    buck_posts();
  }
}

spacing = (mainboard_width - buck_width - ctrl_board_length) / 3;

platform_depth = 2 * ctrl_board_width + 3 * spacing;
platform_holes = [
  [wall + stand_hole_radius, wall + stand_hole_radius],
  [wall + stand_hole_radius, platform_depth - wall - stand_hole_radius],
  [mainboard_width - wall - stand_hole_radius, platform_depth - wall - stand_hole_radius],
  [mainboard_width - wall - stand_hole_radius, wall + stand_hole_radius]
];

module platform() {

  linear_extrude(wall) {
    difference() {
      square([mainboard_width, platform_depth]);
      translate([wall, wall]) {
        square([mainboard_width - 2 * wall, platform_depth - 2 * wall]);
      }
    }
    for(i = [0 : 3]) {
      translate([platform_holes[i][0], platform_holes[i][1]]) {
        difference() {
          circle(r = stand_hole_radius + wall);
          circle(r = stand_hole_radius);
        }
      }
    }
  }

  translate([spacing, spacing, wall]) {
    buck_posts();
      translate([ spacing + buck_width, ctrl_board_width]) {
      rotate(-90) {
        ctrl_board_posts();
      }
    }
    translate([spacing + buck_width, 2 * ctrl_board_width + spacing]) {
      rotate(-90) {
        ctrl_board_posts();
      }
    }
  }
}

module inverted_footprint() {
  difference() {
    translate([wall, wall]) {
      square([mainboard_width - 2 * wall, platform_depth - 2 * wall]);
    }
    projection() {
      platform();
    }
    for(i = [0 : 3]) {
      translate([platform_holes[i][0], platform_holes[i][1]]) {
        circle(r = stand_hole_radius + wall);
      }
    }
  }
}

pattern_horiz_count = 10;
pattern_radius_wall_ratio = 0.85;
pattern_radius = mainboard_width / pattern_horiz_count * pattern_radius_wall_ratio;
pattern_wall = 1.4;

module footprint_pattern(r, w) {
  circle(r = r, $fn = 6);
  translate([3 * (r + w) / 2, (sqrt(3) * r + w) / 2])
    circle(r = r, $fn = 6);
}

pattern_bounding_box = [
  [-3 * pattern_radius/2, -pattern_radius],
  [mainboard_width + pattern_radius, platform_depth + pattern_radius]
];

pattern_moves = [
  [3 * (pattern_radius + wall), 0],
  [0, (sqrt(3) * pattern_radius + wall)]
];

linear_extrude(height = wall) {
  difference() {
    inverted_footprint();
    spray_pattern(pattern_bounding_box, pattern_moves)
      footprint_pattern(pattern_radius, pattern_wall);
  }
}

platform();




