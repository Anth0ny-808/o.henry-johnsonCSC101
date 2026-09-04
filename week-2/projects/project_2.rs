fn main (){
   let toshibaamt =  450_000.00;
   let macamt = 1_500_000.00;
   let hpamt = 750_000.00;
   let dellamt = 2_850_000.00;
   let aceramt = 250_000.00;

   let toshibaqty = 2;
   let macqty = 1;
   let hpqty = 3;
   let dellqty = 3;
   let acerqty = 1;


   let totalqty = toshibaqty + macqty + hpqty + dellqty + acerqty;
   let sum = toshibaamt  + macamt + hpamt + dellamt + aceramt;
   println!("The total sum is {}",totalqty);

   let average = sum / totalqty as f64;

   println!("the  average quantity of the sales record is {}",average)
}

