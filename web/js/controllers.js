'use strict';

function SearchController($scope, $http, $routeParams, $location, localStorageService, $sce)
{
    if (typeof $routeParams.where === 'undefined') {
        $scope.where = '';
    }
    else {
        $scope.where = $routeParams.where;
    }

    if (typeof $routeParams.type === 'undefined') {
        $scope.type = 'all';
    }
    else {
        $scope.type = $routeParams.type;
    }

    if (typeof $routeParams.what === 'undefined') {
        $scope.what = '';
    }
    else {
        $scope.what = $routeParams.what;
    }

    if (typeof $routeParams.wo_hour === 'undefined') {
        $scope.wo_hour = false;
    }
    else {
        $scope.wo_hour = $routeParams.wo_hour;
    }

    if (typeof $routeParams.wifi === 'undefined') {
        $scope.wifi = false;
    }
    else {
        $scope.wifi = $routeParams.wifi;
    }

    if (typeof $routeParams.vegetarian === 'undefined') {
        $scope.vegetarian = false;
    }
    else {
        $scope.vegetarian = $routeParams.vegetarian;
    }

    if (typeof $routeParams.vegan === 'undefined') {
        $scope.vegan = false;
    }
    else {
        $scope.vegan = $routeParams.vegan;
    }

    initMap($scope);
    initType($scope, $http);
    $scope.nodes = [];
    $scope.allNodes = [];
    $scope.progress = 0;
    $scope.searching = false;

    $scope.timeline = function(node) {
        return $sce.trustAsHtml(timeline(node));
    };

    $scope.search = function () {
        var url = '/' + $scope.where + '/' + $scope.type + '/' + $scope.what;
        var params = [];

        if ($scope.wo_hour) {
            params.push('wo_hour');
        }

        if ($scope.wifi) {
            params.push('wifi');
        }

        if ($scope.vegetarian) {
            params.push('vegetarian');
        }

        if ($scope.vegan) {
            params.push('vegan');
        }

        url += '?' + params.join('&');
        $location.url(url.toLowerCase());
    };

    var index = 0;

    $scope.loadMore = function () {
        var limit = Math.min(index + 20, $scope.allNodes.length);

        $scope.nodes = $scope.nodes.concat(
            $scope.allNodes.slice(index, limit)
        );
        index = limit;
    };

    if ($scope.where !== '') {
        search($scope, $http);
    }

    $scope.$watch('box', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined') {
            updateNodes($scope, $http, newValue, localStorageService);
        }
    });

    $scope.$watch('nodes', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined' && newValue.length > 0) {
            updateMap($scope.map, $scope.box, newValue);
        }
    });

    $scope.favorite = function (node) {
        var favorites = localStorageService.get('favorites');

        if (favorites === null) {
            favorites = {};
        }

        if (favorites[node.id] === undefined) {
            favorites[node.id] = true;
            node.favorite = true;
        }
        else {
            delete favorites[node.id];
            node.favorite = false;
        }

        localStorageService.set('favorites', favorites);
    };
}
SearchController.$inject = ['$scope', '$http', '$routeParams', '$location', 'localStorageService', '$sce'];

function initMap($scope)
{
    $scope.map = L.map('map');

    L.tileLayer('https://api.mapbox.com/styles/v1/mapbox/streets-v11/tiles/{z}/{x}/{y}?access_token=' + mapbox_api_key, {
        maxZoom: 18,
        attribution: '',
        id: 'mapbox.streets'
    }).addTo($scope.map);
}

function initType($scope, $http)
{
    $scope.types = [
        'animal_boarding',
        'animal_shelter',
        'arts_centre',
        'atm',
        'baby_hatch',
        'bank',
        'bar',
        'bbq',
        'bench',
        'bicycle parking',
        'bicycle rental',
        'bicycle_repair_station',
        'biergarten',
        'boat_sharing',
        'brothel',
        'bureau de change',
        'bus_station',
        'cafe',
        'car rental',
        'car sharing',
        'car wash',
        'casino',
        'charging_station',
        'cinema',
        'clinic',
        'clock',
        'college',
        'community_centre',
        'courthouse',
        'coworking_space',
        'crematorium',
        'crypt',
        'dentist',
        'doctors',
        'dojo',
        'drinking_water',
        'embassy',
        'ev_charging',
        'fast food',
        'ferry_terminal',
        'firepit',
        'fire_station',
        'food court',
        'fountain',
        'fuel',
        'gambling',
        'game_feeding',
        'grave_yard',
        'grit_bin',
        'gym',
        'hospital',
        'hunting_stand',
        'ice_cream',
        'kindergarten',
        'kneipp_water_cure',
        'library',
        'marketplace',
        'motorcycle parking',
        'nightclub',
        'nursing_home',
        'parking',
        'parking_entrance',
        'parking_space',
        'pharmacy',
        'photo_booth',
        'place of worship',
        'planetarium',
        'police',
        'post_box',
        'post_office',
        'prison',
        'pub',
        'public_bookcase',
        'public_building',
        'ranger_station',
        'recycling',
        'register_office',
        'rescue_station',
        'restaurant',
        'sauna',
        'school',
        'shelter',
        'shower',
        'social_centre',
        'social_facility',
        'stripclub',
        'studio',
        'swingerclub',
        'taxi',
        'telephone',
        'theatre',
        'toilets',
        'townhall',
        'university',
        'vending_machine',
        'veterinary',
        'waste_basket',
        'waste_disposal',
        'watering_place',
        'water_point',
    ];
}

function search($scope, $http)
{
    $scope.searching = true;

    push($scope);
    $http({
        url: 'https://open.mapquestapi.com/nominatim/v1/search.php?key=' + mapquest_api_key + '&format=json&q=' + $scope.where
    }).then(function success(response) {
        var location = response.data[0];

        if (typeof location !== 'undefined') {
            $scope.box = [
                location.boundingbox[0],
                location.boundingbox[2],
                location.boundingbox[1],
                location.boundingbox[3],
            ];
            push($scope);
        }
        else {
            $scope.error = 'Impossible de trouver "' + $scope.where + '"';
            $scope.searching = false;
        }
    }, function error(response) {
        $scope.error = response.data;
        $scope.searching = false;
    });
}

function updateNodes($scope, $http, box, localStorageService)
{
    var filter = '';

    if (!$scope.wo_hour) {
        filter += '["opening_hours"]';
    }

    if ($scope.wifi) {
        filter += '["internet_access"="wlan"]';
    }

    if ($scope.vegetarian) {
        filter += '["diet:vegetarian"="yes"]';
    }

    if ($scope.vegan) {
        filter += '["diet:vegan"="yes"]';
    }

    if ($scope.type !== 'all') {
        filter += '["amenity"="' + $scope.type + '"]';
    }

    if ($scope.what !== '') {
        filter += '["name"~".*' + $scope.what + '.*", i]';
    }

    filter += '(' + box.join() + ');';

    var request = '[out:json][timeout:25]; (way' + filter + ' >; node' + filter + '); out+body;';

    push($scope);
    $http({
        url: 'https://overpass-api.de/api/interpreter?data=' + request
    }).then(function success(response) {
        var nodes = [];
        var elements = response.data.elements;

        if (elements.length !== 0) {
            elements.forEach(function (node) {
                if (
                    typeof node.tags === 'undefined'
                    || (
                        typeof node.tags.name === 'undefined'
                        && typeof node.tags.amenity === 'undefined'
                    )
                ) {
                    return;
                }

                if (node.type === 'way') {
                    replaceRefByNode(node, elements);
                    node.lat = node.nodes[0][0];
                    node.lon = node.nodes[0][1];
                }

                nodes.push({
                    id: node.id,
                    nodes: node.nodes,
                    lat: node.lat,
                    lon: node.lon,
                    name: typeof node.tags.name !== 'undefined' ? node.tags.name : node.tags.amenity,
                    amenity: node.tags.amenity,
                    phone: node.tags.phone,
                    state: getState(node),
                    icon: getIcon(node),
                    vegetarian: node.tags['diet:vegetarian'] === 'yes',
                    vegan: node.tags['diet:vegan'] === 'yes',
                    wifi: node.tags['internet_access'] === 'wlan',
                    favorite: is_favorite(localStorageService, node),
                    tags: node.tags,
                });
            });
        }

        nodes.sort(function (a, b) {
            if (a.favorite && !b.favorite) {
                return -1;
            }
            else if (!a.favorite && b.favorite) {
                return 1;
            }
            else {
                return a.name.localeCompare(b.name);
            }
        });

        $scope.allNodes = nodes;
        $scope.loadMore();
        push($scope);

        $scope.searching = false;
    }, function error(response) {
        $scope.error = response.data;
        $scope.searching = false;
    });
}

function updateMap(map, box, nodes)
{
    nodes.forEach(function (node) {
        var popup = '<div>' +
            '<span class="' + node.icon + '"></span>' +
            node.name +
            '</div>' +
            '<div>' + node.opening_hours + '</div>';

        var color = node.state == 'open' ? 'green' : node.state == 'closed' ? 'red' : 'black';

        if (typeof node.nodes !== 'undefined') {
            L.polygon(node.nodes, {color: color})
                .addTo(map).bindPopup(popup);
        }
        else {
            L.circle([node.lat, node.lon], 5, {color: color})
                .addTo(map).bindPopup(popup);
        }

    });

    setTimeout(function () {
        map.invalidateSize();
        map.fitBounds(nodes);
    }, 10);
}

function getState(node)
{
    var state = '';

    try {
        var opening  = new opening_hours(node.tags.opening_hours, {
            lat: location.lat,
            lon: location.lon,
            address: {
                country_code: 'fr',
            },
        });

        if (opening.getState()) {
            state = 'open';
        }
        else {
            state = 'closed';
        }
    }
    catch (e) {
    }

    return state;
}

function timeline(node)
{
    var content = '';
    var now = new Date();
    var it = getOpening(node);
    var date = it.getDate();
    var days = [
        'Lundi',
        'Mardi',
        'Mercredi',
        'Jeudi',
        'Vendredi',
        'Samedi',
        'Dimanche',
    ];

    content += `
<div class="container legend">
    <div class="row">
        <span class="day col-lg-1"></span>

        <div class="col">`;

    for (var i = 0; i < 24; i++) {
        var classes = [];

        if (i % 5 !== 0) {
            classes.push('label');
        }

        if (i === now.getHours()) {
            classes.push('font-weight-bold');
        }
        else {
            classes.push('text-muted');
        }

        content += `<span class="${classes.join(' ')}">${formatNumber(i)}</span>`;
    }

    content += `</div>
    </div>
</div>`;

    for (var i = 0; i < 7; i++) {
        var classes = ['day', 'col-lg-1'];

        it.setDate(date)

        if (i === now.getDay()) {
            classes.push('font-weight-bold');
        }
        else {
            classes.push('text-muted');
        }

        content += `
<div class="container">
    <div class="row">
        <span class="${classes.join(' ')}">
            ${days[i][0]}<span class="label">${days[i].substring(1)}</span>
        </span>
        <div class="col">
            <div class="progress">`;

        var is_open = it.getState();
        var curdate = date;
        var prevdate = date;

        while (it.advance() && curdate.getTime() - date.getTime() < 24 * 60 * 60 * 1000) {
            curdate = it.getDate();

            var from = prevdate.getTime() - date.getTime();
            var to = curdate.getTime() - date.getTime();

            if (to > 24 * 60 * 60 * 1000) {
                to = 24 * 60 * 60 * 1000;
            }

            from *= 100 / 1000 / 60 / 60 / 24;
            to *= 100 / 1000 / 60 / 60 / 24;

            var size = to - from;
            var color = is_open ? "bg-success" : "bg-danger";
            var legend = (is_open) ? formatTime(prevdate) + ' - ' + formatTime(curdate) : '';
            content += `<div
                role="progressbar"
                style="width: ${size}%;"
                aria-valuenow="${size}"
                aria-valuemin="0"
                aria-valuemax="100"
                class="progress-bar ${color}"
                title="${legend}"
            ><span class="${size < 11 ? 'd-lg-none' : ''}">${legend}</span></div>`;

            prevdate = curdate;
            is_open = it.getState();
        }

        content += `
            </div>
        </div>
    </div>
</div>`;

        date.setDate(date.getDate() + 1);
    }

    return content;
}

function formatTime(date)
{
    return formatNumber(date.getHours()) + ':' + formatNumber(date.getMinutes());
}

function formatNumber(n)
{
    return (n < 10 ? '0' : '') + n;
}

function getOpening(node)
{
    var opening  = new opening_hours(node.tags.opening_hours, {
        lat: location.lat,
        lon: location.lon,
        address: {
            country_code: 'fr',
        },
    });

    var today = new Date();
    return opening.getIterator(today.beginOfWeek());
}

Date.prototype.beginOfWeek = function () {
    return new Date(
        this.getFullYear(),
        this.getMonth(),
        this.getDate() - 7 + this.getDay(),
        0, 0, 0
    );
}

function getIcon(node)
{
    var icon = '';
    var icons = {
        bicycle_parking: 'oc-parking-bicycle',
    };

    if (typeof node.tags.amenity !== 'undefined') {
        if (typeof icons[node.tags.amenity] !== 'undefined') {
            icon = icons[node.tags.amenity];
        }
        else {
            icon = 'oc-' + node.tags.amenity.replace('_', '-');
        }
    }

    return icon;
}

function replaceRefByNode(node, elements)
{
    for (var i = 0; i < node.nodes.length; i++) {
        for (var j = 0; j < elements.length; j++) {
            if (elements[j].id === node.nodes[i]) {
                node.nodes[i] = [
                    elements[j].lat,
                    elements[j].lon,
                ];
                break;
            }
        }
    }
}

function push($scope)
{
    $scope.progress += 25;
}

function is_favorite(localStorageService, node)
{
    var favorites = localStorageService.get('favorites');

    if (favorites === null) {
        return false;
    }

    return favorites[node.id];
}
