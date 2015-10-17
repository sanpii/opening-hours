'use strict';

function SearchController($scope, $http, $routeParams, $location)
{
    if (typeof $routeParams.where === 'undefined') {
        $scope.where = '';
    }
    else {
        $scope.where = $routeParams.where;
    }

    if (typeof $routeParams.what === 'undefined') {
        $scope.what = '';
    }
    else {
        $scope.what = $routeParams.what;
    }

    if (typeof $routeParams.strict === 'undefined') {
        $scope.strict = false;
    }
    else {
        $scope.strict = $routeParams.strict;
    }

    if (typeof $routeParams.wo_hour === 'undefined') {
        $scope.wo_hour = false;
    }
    else {
        $scope.wo_hour = $routeParams.wo_hour;
    }

    initMap($scope);
    $scope.progress = 0;
    $scope.searching = false;

    $scope.search = function () {
        var url = '/' + $scope.where + '/' + $scope.what;
        var params = [];

        if ($scope.strict) {
            params.push('strict');
        }

        if ($scope.wo_hour) {
            params.push('wo_hour');
        }

        url += '?' + params.join('&');
        $location.url(url.toLowerCase());
    };

    if ($scope.where !== '') {
        search($scope, $http);
    }

    $scope.$watch('box', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined') {
            updateNodes($scope, $http, newValue);
        }
    });

    $scope.$watch('nodes', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined' && newValue.length > 0) {
            updateMap($scope.map, $scope.box, newValue);
        }
    });
}
SearchController.$inject = ['$scope', '$http', '$routeParams', '$location'];

function initMap($scope)
{
    $scope.map = L.map('map');

    L.tileLayer('https://api.tiles.mapbox.com/v4/{id}/{z}/{x}/{y}.png?access_token=' + mapbox_api_key, {
        maxZoom: 18,
        attribution: '',
        id: 'mapbox.streets'
    }).addTo($scope.map);
}

function search($scope, $http)
{
    $scope.searching = true;

    push($scope);
    $http({
        url: 'https://open.mapquestapi.com/nominatim/v1/search.php?key=' + mapquest_api_key + '&format=json&q=' + $scope.where
    }).then(function success(response) {
        var location = response.data[0];

        $scope.box = [
            location.boundingbox[0],
            location.boundingbox[2],
            location.boundingbox[1],
            location.boundingbox[3],
        ];
        push($scope);
    });
}

function updateNodes($scope, $http, box)
{
    var node = 'node';
    var request = '[out:json][timeout:25];(';

    if (!$scope.wo_hour) {
        node += '["opening_hours"]';
    }

    if ($scope.what !== '') {
        request += node + '["amenity"="' + $scope.what + '"](' + box.join() + ');';
        if (!$scope.strict) {
            request += node + '["amenity"!~".*"](' + box.join() + ');';
        }
    }
    else {
        request += node + '(' + box.join() + ');';
    }

    request += '); out+body;';

    push($scope);
    $http({
        url: 'https://overpass-api.de/api/interpreter?data=' + request
    }).then(function (response) {
        var nodes = [];

        if (response.data.elements.length === 0) {
            return;
        }

        response.data.elements.forEach(function (node) {
            if (
                typeof node.tags === 'undefined'
                || (
                    typeof node.tags.name === 'undefined'
                    && typeof node.tags.amenity === 'undefined'
                )
            ) {
                return;
            }

            nodes.push({
                lat: node.lat,
                lon: node.lon,
                name: typeof node.tags.name !== 'undefined' ? node.tags.name : node.tags.amenity,
                amenity: node.tags.amenity,
                opening_hours: node.tags.opening_hours,
                state: getState(node),
                icon: getIcon(node),
            });
        });

        $scope.nodes = nodes;
        push($scope);

        $scope.searching = false;
    });
}

function updateMap(map, box, nodes)
{
    nodes.forEach(function (node) {
        L.circle([node.lat, node.lon], 5, {
            color: node.state == 'open' ? 'green' : node.state == 'closed' ? 'red' : 'black',
        }).addTo(map).bindPopup('<div>' +
            '<span class="' + node.icon + '"></span>' +
            node.name +
            '</div>' +
            '<div>' + node.opening_hours + '</div>'
        );
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

function push($scope)
{
    $scope.progress += 25;
}
