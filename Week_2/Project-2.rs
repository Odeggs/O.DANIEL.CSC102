fn main(){
//amount
let t = 450_000;
let m= 1_500_000;
let h = 750_000;
let d = 2_850_000;
let a = 250_000;

//quantity
let tq=2;
let mq = 1;
let hq = 3;
let dq = 3;
let aq = 1;

let  total_quantity = tq+mq+hq+dq+aq;
println!("The total Quantity is {}",total_quantity );

//sum
let sum  = (t*tq) + (m*mq) + (h*hq) + (d*dq) + (a*aq);
println!("THE SUM IS {}",sum );

//average
let _the_average = sum/total_quantity;
println!("The average is {}",_the_average );












}