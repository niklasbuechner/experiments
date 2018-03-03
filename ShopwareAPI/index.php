<?php
include_once "APIClient.php";

$client = new APIClient('http://localhost/viison/shopware/v5.3.7/api/', 'demo', 'tCVL3lTmgFDcbeMH3VgO1lrUZkweeGzdhgf20p9D');

$result = $client->post('articles', [
    'name' => 'Barcode test article',
    'mainDetail' => [
        'additionaltext' => 'Barcode test additionaltext',
        'attribute' => [
            'attr4' => 'Barcode test attribute',
        ],
        'prices' => [
            [
                'customerGroupKey' => 'EK',
                'price' => 50,
            ]
        ],
    ],
    'supplier' => 'Barcode test supplier',
    'tax' => [
        'tax' => 7.00
    ],
]);

var_dump($result);

$client->get('articles/' . $result['data']['id']);
