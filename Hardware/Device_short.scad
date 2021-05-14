$fn=50;

//Builds basic shape with radius
module basic_shape(r=26) {
    //Make it into a circle
    rotate_extrude(convexity = 10) translate([r,0,0]) 
    //Build profile
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

module middle_reduced() {
    //Reduce middle part
    intersection(){
        difference(){
            //Take basic shape
            basic_shape();
            // Remove filling part
            translate([0,0,-0.25]) intersection(){
                scale([0.95,0.95,1]) basic_shape(r=27.65);
                    // Trim part to remove down
                    linear_extrude(2.25) rotate(0.3) square(36);
                    linear_extrude(2.25) rotate(16.98-0.3-90) 
                        square(36);
            }
            //scale([0.9,0.9,1]) translate([0,0,-0.8]) 
                //basic_shape(r=29.4);
        }
        //Cut of Unnecessary parts of the full ring
        linear_extrude(2.25) square(36);
        linear_extrude(2.25) rotate(16.98-90) square(36);
    }
}

module sides_reduced(){
    //Reduce side part
    intersection(){
        difference(){
            //Take "basic" shape
            scale([0.9,0.9,1]) translate([0,0,-0.8]) 
                basic_shape(r=29.4);
            //Remove filling part
            translate([0,0,-1]) intersection(){
                scale([0.85,0.85,1]) basic_shape(r=31.5);
                    linear_extrude(2.25) rotate(-6.11) square(36);
                    linear_extrude(2.25) rotate(23.49-0.4-90) 
                        square(36);
            }
        }
        linear_extrude(2.25) rotate(-6.51) square(36);
        linear_extrude(2.25) rotate(23.49-90) square(36);
    }
}

module sides_only(){
    intersection(){
        sides_reduced();
        linear_extrude(2.25) rotate(16.68) square(36);
    }
    intersection(){
        sides_reduced();
        linear_extrude(2.25) rotate(-90+0.3) square(36);
    }
}

module button_hole() {
    translate([0.2,0,0.5]) intersection(){
        scale([0.89,0.89,1.25]) basic_shape(r=29.9);
        linear_extrude(3) rotate(1.38) square(36);
        linear_extrude(3) rotate(15.60-90) square(36);
    }
}

module screw_hole(){
    difference(){
        union(){
            translate([0,0,0.5]) difference(){
                cylinder(h=0.5,r=0.8);
                translate([0,0,0.25]) cylinder(h=1,r=0.58);
            }
            cylinder(h=0.5,r=0.5);
        }
        translate([0,0,-0.1]) cylinder(h=2,r=0.25);
    }
}

module sides_with_screws(){
    for (i = [0:1]){
        rotate(i * 26.98 - 5) translate([31,0,0]) screw_hole();
    }
    difference() {
        sides_only();
        for (i = [0:1]) {
            rotate(i * 26.98 - 5) translate([31,0,0])
                cylinder(h=3,r=0.58);
        }
    }
}


module switch_mount(l=2.8,r=33.2,a=16.98,h=0.93){
    rotate([0,0,8.5]) translate([0.38,0,-4.3]) rotate([0,-7,0]) rotate([0,0,-8.5]) difference(){
        translate([0,0,h]) intersection(){
            rotate_extrude(convexity = 10) translate([r,0,0])
            square([l,0.145]);
            linear_extrude(2) square(36);
            linear_extrude(2) rotate(16.98-90) square(36);
        }
        rotate(5) translate([r+l-1.3,0,0]) linear_extrude(2) 
            rotate(2.5) square(1.4, center=true);
        rotate(16.98-5) translate([r+l-1.3,0,0])
            rotate(-2.5) linear_extrude(2) square(1.4, center=true);
    }
    rotate(8.3) translate([33.4,0,0]) linear_extrude(h-0.2) // Max 0.2
        square([0.3,10],center=true);
    rotate(8.3) translate([34.6,0,0]) linear_extrude(h+0.07)
        square([2.5,0.3],center=true);
    //rotate(2) translate([r+1,0,0]) cylinder(h=0.93,r=1);
    //rotate(16.98/2) translate([r+l-1.1,0,0]) cylinder(h=0.93,r=1);
    //rotate(15) translate([r+1,0,0]) cylinder(h=0.93,r=1);
}

sides_with_screws();
difference(){
    middle_reduced();
    button_hole();
}
//#button_hole();

switch_mount();