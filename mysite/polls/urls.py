from django.urls import path

from . import views

app_name= 'polls'

urlpatterns = [
    path('', views.index, name='index'),
    #127.0.0.1/polls 
    path('<int:question_id>/', views.detail, name='detail'), 

    #127.0.0.1/polls/1
    path('<int:question_id>/results/', views.results, name='results'),
    
    # ex: /polls/5/vote/
    path('<int:question_id>/vote/', views.vote, name='vote'),
]
