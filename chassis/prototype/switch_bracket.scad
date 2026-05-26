switch_width = 11.6;
switch_height = 13;
beam_depth = 4.15;
beam_height = 4.15;
thickness = 1.4;

brace_width = switch_width * 2;

difference() {
  cube([brace_width, beam_depth + thickness, beam_height + 2 * thickness]);
  translate([0, thickness, thickness]) {
    cube([brace_width, beam_depth, beam_height]);
  }
  translate([(brace_width - switch_width) / 2, thickness, beam_height + thickness]) {
    cube([switch_width, beam_depth, beam_depth]);
  }
}

translate([(brace_width - switch_width) / 2 - thickness, 0, beam_height + 2 * thickness]) {
  difference() {
    cube([switch_width + 2 * thickness, beam_depth + thickness, switch_height + thickness]);
    translate([thickness, thickness, 0]) {
      cube([switch_width, beam_depth, switch_height]);
    }
    translate([thickness + switch_width / 2, thickness, switch_height / 2]) {
      rotate([90]) {
        cylinder(h = thickness, d = switch_width < switch_height ? switch_width : switch_height, $fn = 100);
      }
    }
  }
}
