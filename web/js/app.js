'use strict';

var app = angular.module('opening-hours', ['ngRoute', 'ngResource']);

app.config(['$routeProvider', function($routeProvider) {
    $routeProvider.when('/', {
        templateUrl: 'partials/search.html',
        controller: SearchController
    });
}]);
