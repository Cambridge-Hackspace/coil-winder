use <threads/threads.scad>;

module dd_profile(r_outer, r_inner) {
  intersection() {
    circle(r = r_outer);
    translate([-r_outer, -r_inner]) {
      square([2 * r_outer, 2 * r_inner]);
    }
  }
}

$fn = 100;

difference() {
  RodStart(14, 8, thread_len=16, thread_diam=14);
  linear_extrude(h = 8.8) {
    dd_profile(5 / 2, 3 / 2);
  }
  translate([0, 7, 5.5]) {
    rotate([90, 0, 0]) {
      ScrewThread(4, 14);
    }
  }
}