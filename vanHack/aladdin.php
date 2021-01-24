
<?php
    $mag  =  [2,5,4,3];
    $dist =  [3,4,2,2];
//    $mag  =  [8,4,1,9];
//    $dist =  [10,9,3,5];

    $t_mag = $mag;
    $i = 0;
    $y = 0;
    $resp = false;
    if(count($mag) == count($dist)){
   while($i <= count($mag)-1){
       $bal = $t_mag[$i];
       $y = 0;
       foreach($dist as $dis){
           $y++;
           if($bal >= $dis){
               
               $bal -= $dis;
               if($y <= count($mag)-1){
                   $bal += $mag[$y];
               }else{
                   $resp = true;
               }
           }else{
               $x = array_shift($mag);
               $y = array_shift($dist);
               
               array_push($mag,$x);
               array_push($dist,$y);
               break;
           }
       }
       if($resp == true){
           break;
       }
        $i++;
    
   }
    }
    if($resp == true){
        echo $i;
    }else{
        echo -1;
    }

?>
