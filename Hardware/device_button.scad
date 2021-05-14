$fn=500;

//Builds basic shape with radius
module basic_shape(r=26) {
    //Make it into a circle
    rotate_extrude(convexity = 10, angle=14.06) translate([r,0,0]) 
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

module button() {
    translate([1.4,0,0]) { //intersection(){
        scale([0.86,0.86,0.80]) basic_shape(r=29.9);
        //linear_extrude(3) rotate(0) square(36);
        //linear_extrude(3) rotate(14.06-90) square(36);
    }
}

module button_reduced() {
    difference(){
        button();
        translate([1.5,0.2,-0.2]) 
            scale([0.95,0.95,1]) button();
    }
}

module switch_mount(r=0) {
    translate([0.25,0,0.4]) linear_extrude(1.1)
        square([1,1], center=true);
    rotate (r) difference(){
        translate([0,0,0]) linear_extrude(0.5)
            square([0.45,0.65],center=true);
        translate([0,0,-0.01]) linear_extrude(0.6) 
            square([0.5,0.13],center=true);
        translate([0,0,-0.01]) linear_extrude(0.6) 
            rotate(90) square([0.48,0.13],center=true);
    }
}
button_reduced();
//rotate(3.64) 
//rotate(3.19)
rotate(3.29)
translate([36-1.22,0,0]) switch_mount(2.5);
//rotate(10.62) //9.92
//rotate(13.56)
//rotate(10.17)
rotate(10.27)
translate([36-1.22,0,0]) switch_mount(-2.5);
rotate(6.7) translate([36-4.5,0,0]) linear_extrude(1.58)
    square([0.5,7.2], center=true);


