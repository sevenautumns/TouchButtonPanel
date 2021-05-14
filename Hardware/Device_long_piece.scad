$fn=50;

//Builds basic shape with radius
module long_shape(r=26) {
    rotate_extrude(convexity = 10) translate([r,0,0]) 
    difference(){
        translate([0.5,0,0]) minkowski(){
            difference(){
                square([9,1.5]);
                translate([0,0.3,0]) rotate(20) 
                    translate([-1,0,0]) square([9,4]);
            }
            circle(0.5);
        }
        translate([-1,-12,0]) square(12);
    }
}

module long_shape_reduced() {
    //Remove filling and keep 27.65 Degrees
    intersection(){
        difference(){
            long_shape();
            scale([0.95,0.95,1]) translate([0,0,-0.25]) 
                long_shape(r=27.65);
        }
        linear_extrude(2) square(36);
        linear_extrude(2) rotate(27.65-90) square(36);
    }
}

module screw_hole(){
    union(){
        translate([0,0,1.25]) difference(){
            cylinder(h=0.75, r=0.75);
            translate([0,0,-0.25]) cylinder(h=2, r=0.5);
        }
        difference(){
            translate([0,0,0.75]) 
                cylinder(h=0.75,r=0.5);
            translate([0,0,-0.25]) 
                cylinder(h=2,r=0.25);
        }
        //translate([2.32,0,1.25]) linear_extrude(0.75) 
        //square([3.5,1], center=true);
    }
}

module long_shape_screw() {
    rotate(5.1)
    //rotate(5.18)
    //rotate(0) 
    translate([31,0,0]) 
            screw_hole();
    rotate(22.65) 
    //rotate(27.65)
    translate([31,0,0]) 
            screw_hole();
    difference(){
        long_shape_reduced();
        //rotate(5.18) 
        rotate(5.1) translate([31,0,0])
                cylinder(h=3,r=0.5);
        rotate(22.65) translate([31,0,0])
                cylinder(h=3,r=0.5);
    }
}

long_shape_screw();