function SearchController($scope, $http)
{
    $scope.search = function () {

        $http({
            url: 'http://nominatim.openstreetmap.org/search?format=json&q=' + $scope.where
        }).then(function success(response) {
            box = [
                response.data[0].boundingbox[0],
                response.data[0].boundingbox[2],
                response.data[0].boundingbox[1],
                response.data[0].boundingbox[3],
            ];

            $http({
                url: 'http://overpass-api.de/api/interpreter?data=[out:json][timeout:25];node["opening_hours"](' + box.join() + ');out+body;'
            }).then(function sucess(response) {
                $scope.objects = response.data.elements;
            });
        });
    };
}

SearchController.$inject = ['$scope', '$http'];
