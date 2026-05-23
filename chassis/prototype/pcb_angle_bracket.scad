proto_thickness = 1.55;
board_thickness = 1.45;
height_gap = 6.45;
groove = 4;
wall = 1.6;
length = 20;
height = 37;

scaled_height = height - wall * ((height + sqrt(height * height + length * length)) / length);
scaled_length = length - wall * ((length + sqrt(height * height + length * length)) / height);

rotate([0, 180, -90]) mirror([0, 0, -1]) {

linear_extrude(height = wall) {
  difference() {
    polygon([[0, 0], [length, 0], [0, height]]);
    polygon([
      [wall, wall],
      [scaled_length, wall],
      [wall, scaled_height]
    ]);
  }
}

linear_extrude(height = wall + groove) {
  difference() {
    polygon([
      [0, 0],
      [0, height],
      [2 * wall + proto_thickness, -(height / length) * (2 * wall + proto_thickness) + height],
      [2 * wall + proto_thickness, 2 * wall + board_thickness],
      [((2 * wall + board_thickness) - height) * -(length / height), 2 * wall + board_thickness],
      [length, 0]
    ]);
    translate([wall, height_gap + wall + board_thickness]) {
      square([proto_thickness, height - height_gap - wall - board_thickness]);
    }
    translate([0, wall]) {
      square([length, board_thickness]);
    }
  }
}

}
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  