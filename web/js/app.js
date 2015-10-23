'use strict';

var app = angular.module('opening-hours', ['ngRoute', 'ngResource', 'infinite-scroll']);

app.config(['$routeProvider', function($routeProvider) {
    $routeProvider.when('/about', {
        templateUrl: 'partials/about.html'
    });

    $routeProvider.when('/:where?/:type?/:what?', {
        templateUrl: 'partials/search.html',
        controller: SearchController
    });
}]);
