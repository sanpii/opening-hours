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

    $scope.search = function () {
        $location.url('/' + $scope.where + '/' + $scope.what);
    };

    if ($scope.where !== '') {
        search($scope, $http);
    }

    $scope.$watch('box', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined') {
            updateList($scope, $http, newValue);
        }
    });

    $scope.$watch('nodes', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined') {
            updateNodes($scope.map, newValue);
        }
    });

    initMap($scope);

    $scope.$watch('box', function (newValue, oldValue) {
        if (typeof newValue !== 'undefined') {
            updateMap($scope.map, newValue);
        }
    });
}
SearchController.$inject = ['$scope', '$http', '$routeParams', '$location'];

function initMap($scope)
{
    $scope.map = L.map('map');

    L.tileLayer('https://api.tiles.mapbox.com/v4/{id}/{z}/{x}/{y}.png?access_token=pk.eyJ1IjoibWFwYm94IiwiYSI6IjZjNmRjNzk3ZmE2MTcwOTEwMGY0MzU3YjUzOWFmNWZhIn0.Y8bhBaUMqFiPrDRW9hieoQ', {
        maxZoom: 18,
        attribution: 'Map data &copy; <a href="http://openstreetmap.org">OpenStreetMap</a> contributors, ' +
        '<a href="http://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, ' +
        'Imagery © <a href="http://mapbox.com">Mapbox</a>',
        id: 'mapbox.streets'
    }).addTo($scope.map);
}

function search($scope, $http)
{
    $http({
        url: 'http://nominatim.openstreetmap.org/search?format=json&q=' + $scope.where
    }).then(function success(response) {
        var location = response.data[0];

        $scope.box = [
            location.boundingbox[0],
            location.boundingbox[2],
            location.boundingbox[1],
            location.boundingbox[3],
        ];
    });
}

function updateList($scope, $http, box)
{
    $http({
        url: 'http://overpass-api.de/api/interpreter?data=[out:json][timeout:25];node["opening_hours"](' + box.join() + ');out+body;'
    }).then(function sucess(response) {
        $scope.nodes = [];

        response.data.elements.forEach(function (node) {
            if (
                typeof node.tags.name === 'undefined'
            && typeof node.tags.amenity === 'undefined'
            ) {
                return;
            }

            $scope.nodes.push({
                lat: node.lat,
                lon: node.lon,
                name: typeof node.tags.name !== 'undefined' ? node.tags.name : node.tags.amenity,
                amenity: node.tags.amenity,
                opening_hours: node.tags.opening_hours,
                state: getState(node),
                icon: getIcon(node),
            });
        });
    });
}

function updateMap(map, box)
{
    map.fitBounds([
        [box[0], box[1]],
        [box[2], box[3]],
    ]);
}

function updateNodes(map, nodes)
{
    nodes.forEach(function (node) {
        L.circle([node.lat, node.lon], 5, {
            color: node.state == 'open' ? 'green' : node.state == 'closed' ? 'red' : 'black',
        }).addTo(map);
    });
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
