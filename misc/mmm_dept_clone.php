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
<!-- without pointer -->
<?php
$bal = [];
$dpt = [];
$ph = ['Femi'=>19000,'Kunle'=>5000,'Wole'=>3000,'Dav'=>2000,'Kif'=>1000];
$gh = ['Wale'=>19000,'Tunde'=>6000,'Kiki'=>1000,'Dare'=>3000];

echo '<br>';
foreach ($ph as $key1=>$value1){
    foreach ($gh as $key2=>$value2){
        if($value1 != 0 && $value2 != 0){
            if($value1 > $value2){
                $pay = $value2;
                $value1 = $value1 - $value2;
                $value2 = 0;
                $ph[$key1] = $value1;
                $gh[$key2] = $value2;
                echo $key1. ' pays '.$key2." $pay".'<br>';
            }else{
                $pay = $value1;
                $value2 = $value2 - $value1;
                $value1 = 0;
                $ph[$key1] = $value1;
                $gh[$key2] = $value2;
                echo $key1. ' pays '.$key2." $pay".'<br>';
            }
        }
    }
}
echo '<br>';
//fix remaining error


?>