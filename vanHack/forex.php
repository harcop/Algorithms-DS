<?php


$date = new DateTime('1-January-2000');
$date2 = new DateTime('22-February-2000');
$date = $date2->diff($date);
echo $date_diff = $date->days;
echo '<br>';


echo $day = date("d-m-Y", strtotime("1-January-2000"));
echo '<br>';
for($i=0; $i<= $date_diff;$i++){
    $daf = date("d-m-Y", strtotime("$day, +$i day"));
    $dayd = new DateTime($daf);
    if($dayd->format("l") == 'Monday'){
         $resp[] = $dayd->format("j-F-Y");
    }
    
}
echo '<pre>';
print_r($resp);
echo '</pre>';

foreach($resp as $day){
    $del = file_get_contents("https://jsonmock.hackerrank.com/api/stocks/?date=$day");
    $det = json_decode($del,true);
//    print_r($det['data']['0']['date']);
    if(($det['data']) != NULL){
        echo "{$det['data']['0']['date']} "."{$det['data']['0']['open']} "."{$det['data']['0']['close']}"."\n";
    }
}

echo '<br>';
echo $dater = date('d-m-y');
echo '<br>';
echo $day4 = date("d-m-Y", strtotime("$dater, -4 month"));
$day4 = new DateTime($day4);
$dater = new DateTime($dater);
echo "<br>";
echo $day4->diff($dater)->m;

echo $dat = date('d-M-Y-W');
$date_new = date('Y-m-d');
echo $date_past = date('Y-m',strtotime("$date_new, -4 month"));
$day4 = new DateTime($date_new);
$dater = new DateTime($date_past);
echo $day4->diff($dater)->m;


echo $date_now = date('Y-m-d');
echo '<br>';
$date_inv = '2019-09-02';
               echo $date_inv = date('Y-m-d',strtotime("$date_inv"));
                $date_now = new DateTime($date_now);
                $date_inv = new DateTime($date_inv);
//                $date_now->format('Y-m-d');
//                $date_inv->format('Y-m-d');
                $date_diff = $date_inv->diff($date_now);
echo '<br>';
               echo  $date_dif = $date_now->diff($date_inv)->m;
echo '<br>';
                echo $mon_diff = $date_diff->m;
if($mon_diff == 1){
    echo 'good';
}
?>