<?php

$ph = ['Femi'=>19000,'Kunle'=>5000,'Wole'=>3000,'Dav'=>2000,'Kif'=>1000];
$gh = ['Wale'=>19000,'Tunde'=>6000,'Kiki'=>1000,'Dare'=>3000];

foreach($ph as $payer=>&$gift){
    foreach($gh as $payee=>&$coll){
       if($gift != 0 && $coll != 0){
           if($gift > $coll){
               $gift -= $coll;
               echo "$payer pays $payee $coll <br>";
               $coll = 0;
           }else{
               $coll -= $gift;
               echo "$payer pays $payee $gift <br>";
               $gift = 0;
           }
       } 
    } 
}
//print_r($ph);
?>