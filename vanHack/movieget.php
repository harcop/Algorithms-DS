<?php
    $substr = 'maze';
//    $url = "https://google.com";
//        $url = 'http://localhost:1234/file.php';
   $url = "https://jsonmock.hackerrank.com/api/movies/search/?Title=$substr";
    $url = "http://twitter.com";
   $ch = curl_init($url);
   curl_setopt($ch,CURLOPT_HTTPHEADER,['Content-Type:appplication/json']);
   curl_setopt($ch,CURLOPT_RETURNTRANSFER,true);
   curl_setopt($ch,CURLOPT_CUSTOMREQUEST,'GET');
   $result = curl_exec($ch);
   $httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
   curl_close($ch);
   if($httpCode == 200){
       $result = json_decode($result,true);
       print($result['total']);
   }

$resp = file_get_contents($url);
echo $resp;
$resp = json_decode($resp,true);
       print($resp['total']);
?>